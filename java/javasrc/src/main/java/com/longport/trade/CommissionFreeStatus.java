package com.longport.trade;

/**
 * Commission-free status
 */
public enum CommissionFreeStatus {
    /** Unknown */
    Unknown,
    /** Not applicable */
    None,
    /** Commission-free amount calculated */
    Calculated,
    /** Pending */
    Pending,
    /** Commission-free amount ready */
    Ready,
}
