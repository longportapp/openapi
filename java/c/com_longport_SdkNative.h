/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class com_longport_SdkNative */

#ifndef _Included_com_longport_SdkNative
#define _Included_com_longport_SdkNative
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     com_longport_SdkNative
 * Method:    init
 * Signature: ()V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_init
  (JNIEnv *, jclass);

/*
 * Class:     com_longport_SdkNative
 * Method:    newHttpClient
 * Signature: (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)J
 */
JNIEXPORT jlong JNICALL Java_com_longport_SdkNative_newHttpClient
  (JNIEnv *, jclass, jstring, jstring, jstring, jstring);

/*
 * Class:     com_longport_SdkNative
 * Method:    newHttpClientFromEnv
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_longport_SdkNative_newHttpClientFromEnv
  (JNIEnv *, jclass);

/*
 * Class:     com_longport_SdkNative
 * Method:    freeHttpClient
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_freeHttpClient
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    httpClientRequest
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_httpClientRequest
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    newConfig
 * Signature: (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lcom/longport/Language;ZLcom/longport/PushCandlestickMode;)J
 */
JNIEXPORT jlong JNICALL Java_com_longport_SdkNative_newConfig
  (JNIEnv *, jclass, jstring, jstring, jstring, jstring, jstring, jstring, jobject, jboolean, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    newConfigFromEnv
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_longport_SdkNative_newConfigFromEnv
  (JNIEnv *, jclass);

/*
 * Class:     com_longport_SdkNative
 * Method:    configRefreshAccessToken
 * Signature: (JLjava/time/OffsetDateTime;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_configRefreshAccessToken
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    freeConfig
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_freeConfig
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    newQuoteContext
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_newQuoteContext
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    freeQuoteContext
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_freeQuoteContext
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextGetMemberId
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_com_longport_SdkNative_quoteContextGetMemberId
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextGetQuoteLevel
 * Signature: (J)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_com_longport_SdkNative_quoteContextGetQuoteLevel
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextGetQuotePackageDetails
 * Signature: (J)[Lcom/longport/quote/QuotePackageDetail;
 */
JNIEXPORT jobjectArray JNICALL Java_com_longport_SdkNative_quoteContextGetQuotePackageDetails
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSetOnQuote
 * Signature: (JLcom/longport/quote/QuoteHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSetOnQuote
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSetOnDepth
 * Signature: (JLcom/longport/quote/DepthHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSetOnDepth
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSetOnBrokers
 * Signature: (JLcom/longport/quote/BrokersHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSetOnBrokers
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSetOnTrades
 * Signature: (JLcom/longport/quote/TradesHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSetOnTrades
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSetOnCandlestick
 * Signature: (JLcom/longport/quote/CandlestickHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSetOnCandlestick
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSubscribe
 * Signature: (J[Ljava/lang/String;IZLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jint, jboolean, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextUnsubscribe
 * Signature: (J[Ljava/lang/String;ILcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextUnsubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jint, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSubscribeCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longport/quote/Period;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSubscribeCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextUnsubscribeCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longport/quote/Period;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextUnsubscribeCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSubscriptions
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSubscriptions
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextStaticInfo
 * Signature: (J[Ljava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextStaticInfo
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextQuote
 * Signature: (J[Ljava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextOptionQuote
 * Signature: (J[Ljava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextOptionQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextWarrantQuote
 * Signature: (J[Ljava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextWarrantQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextDepth
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextDepth
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextBrokers
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextBrokers
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextParticipants
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextParticipants
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextTrades
 * Signature: (JLjava/lang/String;ILcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextTrades
  (JNIEnv *, jclass, jlong, jstring, jint, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextIntraday
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextIntraday
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longport/quote/Period;ILcom/longport/quote/AdjustType;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jint, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextHistoryCandlesticksByOffset
 * Signature: (JLjava/lang/String;Lcom/longport/quote/Period;Lcom/longport/quote/AdjustType;ZLjava/time/LocalDateTime;ILcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextHistoryCandlesticksByOffset
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject, jboolean, jobject, jint, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextHistoryCandlesticksByDate
 * Signature: (JLjava/lang/String;Lcom/longport/quote/Period;Lcom/longport/quote/AdjustType;Ljava/time/LocalDate;Ljava/time/LocalDate;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextHistoryCandlesticksByDate
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject, jobject, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextOptionChainExpiryDateList
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextOptionChainExpiryDateList
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextOptionChainInfoByDate
 * Signature: (JLjava/lang/String;Ljava/time/LocalDate;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextOptionChainInfoByDate
  (JNIEnv *, jclass, jlong, jstring, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextWarrantIssuers
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextWarrantIssuers
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextWarrantList
 * Signature: (JLcom/longport/quote/QueryWarrantOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextWarrantList
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextTradingSession
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextTradingSession
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextTradingDays
 * Signature: (JLcom/longport/Market;Ljava/time/LocalDate;Ljava/time/LocalDate;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextTradingDays
  (JNIEnv *, jclass, jlong, jobject, jobject, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextCapitalFlow
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextCapitalFlow
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextCapitalDistribution
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextCapitalDistribution
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextCalcIndexes
 * Signature: (J[Ljava/lang/String;[Lcom/longport/quote/CalcIndex;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextCalcIndexes
  (JNIEnv *, jclass, jlong, jobjectArray, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextWatchlist
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextWatchlist
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextCreateWatchlistGroup
 * Signature: (JLcom/longport/quote/CreateWatchlistGroup;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextCreateWatchlistGroup
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextDeleteWatchlistGroup
 * Signature: (JLcom/longport/quote/DeleteWatchlistGroup;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextDeleteWatchlistGroup
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextUpdateWatchlistGroup
 * Signature: (JLcom/longport/quote/UpdateWatchlistGroup;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextUpdateWatchlistGroup
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextSecurityList
 * Signature: (JLcom/longport/Market;Lcom/longport/quote/SecurityListCategory;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextSecurityList
  (JNIEnv *, jclass, jlong, jobject, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextRealtimeQuote
 * Signature: (J[Ljava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextRealtimeQuote
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextRealtimeDepth
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextRealtimeDepth
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextRealtimeBrokers
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextRealtimeBrokers
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextRealtimeTrades
 * Signature: (JLjava/lang/String;ILcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextRealtimeTrades
  (JNIEnv *, jclass, jlong, jstring, jint, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    quoteContextRealtimeCandlesticks
 * Signature: (JLjava/lang/String;Lcom/longport/quote/Period;ILcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_quoteContextRealtimeCandlesticks
  (JNIEnv *, jclass, jlong, jstring, jobject, jint, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    newTradeContext
 * Signature: (JLcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_newTradeContext
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    freeTradeContext
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_freeTradeContext
  (JNIEnv *, jclass, jlong);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextSetOnOrderChanged
 * Signature: (JLcom/longport/trade/OrderChangedHandler;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextSetOnOrderChanged
  (JNIEnv *, jclass, jlong, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextSubscribe
 * Signature: (J[Lcom/longport/trade/TopicType;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextSubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextUnsubscribe
 * Signature: (J[Lcom/longport/trade/TopicType;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextUnsubscribe
  (JNIEnv *, jclass, jlong, jobjectArray, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextHistoryExecutions
 * Signature: (JLcom/longport/trade/GetHistoryExecutionsOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextHistoryExecutions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextTodayExecutions
 * Signature: (JLcom/longport/trade/GetTodayExecutionsOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextTodayExecutions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextHistoryOrders
 * Signature: (JLcom/longport/trade/GetHistoryOrdersOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextHistoryOrders
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextTodayOrders
 * Signature: (JLcom/longport/trade/GetTodayOrdersOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextTodayOrders
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextReplaceOrder
 * Signature: (JLcom/longport/trade/ReplaceOrderOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextReplaceOrder
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextSubmitOrder
 * Signature: (JLcom/longport/trade/SubmitOrderOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextSubmitOrder
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextCancelOrder
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextCancelOrder
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextAccountBalance
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextAccountBalance
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextCashFlow
 * Signature: (JLcom/longport/trade/GetCashFlowOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextCashFlow
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextFundPositions
 * Signature: (JLcom/longport/trade/GetFundPositionsOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextFundPositions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextStockPositions
 * Signature: (JLcom/longport/trade/GetStockPositionsOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextStockPositions
  (JNIEnv *, jclass, jlong, jobject, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextMarginRatio
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextMarginRatio
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextOrderDetail
 * Signature: (JLjava/lang/String;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextOrderDetail
  (JNIEnv *, jclass, jlong, jstring, jobject);

/*
 * Class:     com_longport_SdkNative
 * Method:    tradeContextEstimateMaxPurchaseQuantity
 * Signature: (JLcom/longport/trade/EstimateMaxPurchaseQuantityOptions;Lcom/longport/AsyncCallback;)V
 */
JNIEXPORT void JNICALL Java_com_longport_SdkNative_tradeContextEstimateMaxPurchaseQuantity
  (JNIEnv *, jclass, jlong, jobject, jobject);

#ifdef __cplusplus
}
#endif
#endif
