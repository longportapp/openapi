package com.longport.quote;

import java.math.BigDecimal;
import java.util.Arrays;

/**
 * Basic (static) information of a security.
 */
public class SecurityStaticInfo {
    private String symbol;
    private String nameCn;
    private String nameEn;
    private String nameHk;
    private String exchange;
    private String currency;
    private int lotSize;
    private long totalShares;
    private long circulatingShares;
    private long hkShares;
    private BigDecimal eps;
    private BigDecimal epsTtm;
    private BigDecimal bps;
    private BigDecimal dividendYield;
    private DerivativeType[] stockDerivatives;
    private SecurityBoard board;

    /**
     * Returns the security code.
     *
     * @return the security code
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the security name in simplified Chinese.
     *
     * @return the security name in simplified Chinese
     */
    public String getNameCn() {
        return nameCn;
    }

    /**
     * Returns the security name in English.
     *
     * @return the security name in English
     */
    public String getNameEn() {
        return nameEn;
    }

    /**
     * Returns the security name in traditional Chinese.
     *
     * @return the security name in traditional Chinese
     */
    public String getNameHk() {
        return nameHk;
    }

    /**
     * Returns the exchange the security is listed on.
     *
     * @return the exchange
     */
    public String getExchange() {
        return exchange;
    }

    /**
     * Returns the trading currency.
     *
     * @return the trading currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the lot size.
     *
     * @return the lot size
     */
    public int getLotSize() {
        return lotSize;
    }

    /**
     * Returns the total number of issued shares.
     *
     * @return the total number of issued shares
     */
    public long getTotalShares() {
        return totalShares;
    }

    /**
     * Returns the number of circulating shares.
     *
     * @return the number of circulating shares
     */
    public long getCirculatingShares() {
        return circulatingShares;
    }

    /**
     * Returns the number of HK shares (only for HK stocks).
     *
     * @return the number of HK shares
     */
    public long getHkShares() {
        return hkShares;
    }

    /**
     * Returns the earnings per share.
     *
     * @return the earnings per share
     */
    public BigDecimal getEps() {
        return eps;
    }

    /**
     * Returns the earnings per share (TTM).
     *
     * @return the earnings per share (TTM)
     */
    public BigDecimal getEpsTtm() {
        return epsTtm;
    }

    /**
     * Returns the net assets per share.
     *
     * @return the net assets per share
     */
    public BigDecimal getBps() {
        return bps;
    }

    /**
     * Returns the dividend (per share), <b>not</b> the dividend yield (ratio).
     *
     * @return the dividend (per share)
     */
    public BigDecimal getDividendYield() {
        return dividendYield;
    }

    /**
     * Returns the supported derivative types for this security.
     *
     * @return the supported derivative types
     */
    public DerivativeType[] getStockDerivatives() {
        return stockDerivatives;
    }

    /**
     * Returns the security board.
     *
     * @return the security board
     */
    public SecurityBoard getBoard() {
        return board;
    }

    @Override
    public String toString() {
        return "SecurityStaticInfo [board=" + board + ", bps=" + bps + ", circulatingShares=" + circulatingShares
                + ", currency=" + currency + ", dividendYield=" + dividendYield + ", eps=" + eps + ", epsTtm=" + epsTtm
                + ", exchange=" + exchange + ", hkShares=" + hkShares + ", lotSize=" + lotSize + ", nameCn=" + nameCn
                + ", nameEn=" + nameEn + ", nameHk=" + nameHk + ", stockDerivatives="
                + Arrays.toString(stockDerivatives) + ", symbol=" + symbol + ", totalShares=" + totalShares + "]";
    }

}
