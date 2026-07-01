package com.longport.quote;

/**
 * Security basic information.
 */
public class Security {
    private String symbol;
    private String nameCn;
    private String nameEn;
    private String nameHk;

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

    @Override
    public String toString() {
        return "Security [symbol=" + symbol + ", nameCn=" + nameCn + ", nameEn=" + nameEn + ", nameHk=" + nameHk + "]";
    }
}