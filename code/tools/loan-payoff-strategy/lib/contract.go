package interest

import (
	"fmt"
	"math"
)

type Contract struct {
	Principal float64
	// Annual Interest Rate. A value between 0 and 1.
	AnnualIR           float64
	TermYears          int
	MonthlyIRCompounds bool
}

// Calculate the amount remaining after letting the contract run its course.
// Might be negative. In that case, the monthly installment is too high and can
// be reduced.
func (c *Contract) Simulate(installment func(MonthState) float64) Outcome {
	monthly_ir := c.monthly_ir()
	m := MonthState{Principal: c.Principal}
	total_paid := 0.

	for range c.TermYears * 12 {
		m.Interest = m.Principal * monthly_ir
		installment := installment(m)

		m.Principal += m.Interest  // Charge that month's interest.
		m.Principal -= installment // Take the monthly installment.
		total_paid += installment  // Track the total paid.
	}
	return Outcome{total_paid, m.Principal}
}

func (c *Contract) TotalMonths() int {
	return c.TermYears * 12
}

func (c *Contract) XAxis() []string {
	xaxis := make([]string, c.TotalMonths()+12)
	for year := range c.TermYears + 1 {
		xaxis[year*12] = fmt.Sprintf("%d", year)
	}
	return xaxis
}

func (c *Contract) monthly_ir() float64 {
	if c.MonthlyIRCompounds {
		return math.Pow(1.+c.AnnualIR, 1./12.) - 1.
	} else {
		return c.AnnualIR / 12.
	}
}

// Run a binary search to solve for the monthly installment needed. Also returns
// the number of iterations used in the binary search. The search ends when the
// final outcome of the contract is left with a remaining principal that falls
// between the two floats supplied. A good default should be -1.0 and 0.0.
// That is, we over-pay by a few cents.
func (c *Contract) BinarySearch(ok_low float64, ok_high float64) (float64, int) {
	if ok_low > ok_high {
		panic("Use a better ok range.")
	}
	sim, bs_low, bs_high := 0, 0.0, c.Principal
	for {
		sim++
		monthly_installment := (bs_low + bs_high) / 2.0
		outcome := c.Simulate(func(MonthState) float64 { return monthly_installment })
		fmt.Printf("%12.2f -> %14.2f\n", monthly_installment, outcome.Principal)

		if outcome.Principal < ok_low {
			bs_high = monthly_installment
		} else if outcome.Principal > ok_high {
			bs_low = monthly_installment
		} else {
			// ok_low <= remaining_principal <= ok_high. All's ok.
			return monthly_installment, sim
		}
	}
}
