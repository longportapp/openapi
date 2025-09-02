#include <iostream>

#include "longport.h"
#include "longport.hpp"
#include "ta_libc.h"

/* 
	This example requires the installation of TA-LIB library
	The download address is: https://ta-lib.org/
*/

using namespace longport;
using namespace longport::quote;

int main()
{
	longport::Config config;
	longport::Status status = longport::Config::from_env(config);

	TA_RetCode retCode;
	retCode = TA_Initialize();
	if (retCode != TA_SUCCESS)
	{
		std::cout << "Cannot initialize TA-Lib !\n";
		return -1;
	}
	
	if (!status) {
		std::cout << "failed to load configuration from environment: "
			<< status.message() << std::endl;
		return -1;
	}
	QuoteContext::create(config, [&](auto res) {
		if (!res) {
			std::cout << "failed to create quote context: " << res.status().message()
				<< std::endl;
			return;
		}

		res.context().history_candlesticks_by_offset("700.HK", Period::Min1, AdjustType::NoAdjust, false, std::nullopt, 50, TradeSessions::Intraday, [&](auto res) {
			if (!res) {
				std::cout << "failed to request history candlesticks: "
					<< res.status().message() << std::endl;
				return;
			}
			// MACD Data initialization
			std::vector<TA_Real> InReal, OutMACD, OutMACDSignal, OutMACDHist;
			int Out_beg = 0;
			int	OutElements = 0;
			// Obtain a certain amount of historical data
			for (auto it = res->cbegin(); it != res->cend(); ++it) {
				InReal.push_back((double)it->close);
				std::cout << " close=" << (double)it->close
					<< " open=" << (double)it->open
					<< " low=" << (double)it->low
					<< " high=" << (double)it->high
					<< " volume=" << (int64_t)it->volume
					<< " turnover=" << (double)it->turnover
					<< " timestamp=" << (int64_t)it->timestamp << std::endl;
			}
			// dynamically allocate output arrays
			// 12, 26, 9, TA_MAType_EMA - are defaults for MACD
			int data_size = InReal.size();
			int OutSize = data_size - TA_MACDEXT_Lookback(12, TA_MAType_EMA, 26, TA_MAType_EMA, 9, TA_MAType_EMA);
			if (OutSize != 0) {
				OutMACD.resize(OutSize);
				OutMACDSignal.resize(OutSize);
				OutMACDHist.resize(OutSize);
			}
			int Ret_code = TA_MACDEXT(0, data_size - 1, InReal.data(), 12, TA_MAType_EMA, 26, TA_MAType_EMA, 9, TA_MAType_EMA, &Out_beg, &OutElements, OutMACD.data(), OutMACDSignal.data(), OutMACDHist.data());
			int lastElement = OutElements - 1;
			if (Ret_code == TA_SUCCESS)
			{
				std::cout << "-------------------------" << std::endl;
				std::cout << "MACD Calculation Result: " << std::endl;
				std::cout << "MACD: " << OutMACDHist[lastElement] * 2 << std::endl;   // The parameter of the Chinese stock market is multiplied by twice!
				std::cout << "DIF: " << OutMACD[lastElement] << std::endl;
				std::cout << "DEA: " << OutMACDSignal[lastElement] << std::endl;
				std::cout << "-------------------------" << std::endl;
			}
			else
			{
				std::cout << "Failed to calculate MACD: " << Ret_code << std::endl;
			}

			});
		});

	std::cin.get();
	retCode = TA_Shutdown();
	if (retCode != TA_SUCCESS)
	{
		std::cout << "Cannot TA_Shutdown TA-Lib !\n";
	}
	return 0;
}
