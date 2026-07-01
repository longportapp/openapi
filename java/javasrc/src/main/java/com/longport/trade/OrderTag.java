package com.longport.trade;

/**
 * Order tag
 */
public enum OrderTag {
    /** Unknown */
    Unknown,
    /** Normal order */
    Normal,
    /** Long-term order */
    LongTerm,
    /** Grey market order */
    Grey,
    /** Margin call order */
    MarginCall,
    /** Offline order */
    Offline,
    /** Creditor order */
    Creditor,
    /** Debtor order */
    Debtor,
    /** Non-exercise order */
    NonExercise,
    /** Allocated sub order */
    AllocatedSub,
}
