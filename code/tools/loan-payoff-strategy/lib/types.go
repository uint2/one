package interest

type MonthState struct {
	// Remaining principal in dollars.
	Principal float64
	// That month's interest, in dollars.
	Interest float64
}

type Outcome struct {
	TotalPaid float64
	// Remaining principal after all is said and done.
	Principal float64
}
