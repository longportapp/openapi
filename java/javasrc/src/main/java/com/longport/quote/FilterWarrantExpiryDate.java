package com.longport.quote;

/**
 * Filter warrant expiry date
 */
public enum FilterWarrantExpiryDate {
    /** Less than 3 months */
    LT_3,
    /** 3 - 6 months */
    Between_3_6,
    /** 6 - 12 months */
    Between_6_12,
    /** Greater than 12 months */
    GT_12,
}
