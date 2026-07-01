package com.longport.quote;

/**
 * Warrant issuer information.
 */
public class IssuerInfo {
    private int issuerId;
    private String nameCn;
    private String nameEn;
    private String nameHk;

    /**
     * Returns the issuer ID.
     *
     * @return the issuer ID
     */
    public int getIssuerId() {
        return issuerId;
    }

    /**
     * Returns the issuer name in simplified Chinese.
     *
     * @return the issuer name in simplified Chinese
     */
    public String getNameCn() {
        return nameCn;
    }

    /**
     * Returns the issuer name in English.
     *
     * @return the issuer name in English
     */
    public String getNameEn() {
        return nameEn;
    }

    /**
     * Returns the issuer name in traditional Chinese.
     *
     * @return the issuer name in traditional Chinese
     */
    public String getNameHk() {
        return nameHk;
    }

    @Override
    public String toString() {
        return "IssuerInfo [issuerId=" + issuerId + ", nameCn=" + nameCn + ", nameEn=" + nameEn + ", nameHk=" + nameHk
                + "]";
    }
}
