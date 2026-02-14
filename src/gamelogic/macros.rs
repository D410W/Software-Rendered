#[macro_export]
macro_rules! return_if {
    // Pattern 1: Just the condition (returns nothing/unit)
    ($cond:expr) => {
        if $cond {
            return;
        }
    };
    
    // Pattern 2: Condition and a specific value to return
    ($cond:expr, $value:expr) => {
        if $cond {
            return $value;
        }
    };
}

#[macro_export]
macro_rules! continue_if {
    ($cond:expr) => {
        if $cond { continue; }
    }
}
