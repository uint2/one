package main

import (
	. "interest/lib"

	"fmt"
	"math"
	"os"

	"github.com/go-echarts/go-echarts/v2/charts"
	"github.com/go-echarts/go-echarts/v2/opts"
)

/*

Assumptions:
 * Everything happens on the 1st of each month. The monthly payments, and the
   calculation of interest.
   Reason:
   Since everything is periodic it doesn't matter too much exactly when these
   events happen relative to each other.

Conclusions:
 * The amount paid for interest per month has to be strictly less than the
   recommended amount. This can easily be proven by basic reasoning. If the
   accumulated interest monthly is already greater than the recommended monthly
   payment, then the principal will never be paid off.
*/

// Rounds floats to 2 decimal places, for prettifying money values.
func round2(x float64) float64 {
	return math.Round(x*100.) / 100.
}

func toData(x []float64) []opts.LineData {
	y := make([]opts.LineData, len(x))
	for i := range len(x) {
		y[i] = opts.LineData{Value: round2(x[i])}
	}
	return y
}

func color(hex string) charts.SeriesOpts {
	return charts.WithItemStyleOpts(opts.ItemStyle{Color: hex})
}

func main() {
	contract := Contract{
		Principal:          1_100_000,
		AnnualIR:           0.10,
		TermYears:          15,
		MonthlyIRCompounds: true,
	}

	monthly_installment, sims := contract.BinarySearch(-1, 0)

	fmt.Printf("Binary search done! (%d simulations used)\n", sims)

	chart := charts.NewLine()
	chart.SetGlobalOptions(charts.WithXAxisOpts(opts.XAxis{Name: "Years"}))
	chart.SetGlobalOptions(charts.WithYAxisOpts(opts.YAxis{
		Name:      "Monthly",
		AxisLabel: &opts.AxisLabel{Formatter: "${value}"}}))

	var strat1, strat2 Outcome

	{ // Case 1: Pay a flat rate throughout.
		installments := make([]float64, 0)
		interests := make([]float64, 0)
		strat1 = contract.Simulate(func(m MonthState) float64 {
			installments = append(installments, round2(monthly_installment))
			interests = append(interests, m.Interest)
			return monthly_installment
		})
		chart.AddSeries("Fixed monthly installment", toData(installments), color(BLUE_500))
		chart.AddSeries("Fixed monthly installment (interest)", toData(interests), color(BLUE_300))

		fmt.Printf("Flat monthly installment: $%.2f\n", monthly_installment)
		fmt.Printf("Total paid: \x1b[32m$%.2f\x1b[m\n", strat1.TotalPaid)
	}
	{ // Case 2: Attack the principal at a constant rate.
		flat_principal_deduct := contract.Principal / float64(contract.TotalMonths())
		installments := make([]float64, 0)
		interests := make([]float64, 0)
		strat2 = contract.Simulate(func(m MonthState) float64 {
			installment := m.Interest + flat_principal_deduct
			installments = append(installments, installment)
			interests = append(interests, m.Interest)
			return installment
		})
		chart.AddSeries("Fixed principal deduct", toData(installments), color(ORANGE_500))
		chart.AddSeries("Fixed principal deduct (interest)", toData(interests), color(ORANGE_300))

		fmt.Printf("Flat monthly principal deduct: $%.2f\n", flat_principal_deduct)
		fmt.Printf("Total paid: \x1b[32m$%.2f\x1b[m\n", strat2.TotalPaid)
	}

	abs_diff := strat1.TotalPaid - strat2.TotalPaid
	pct_diff := abs_diff / contract.Principal * 100
	fmt.Printf("Difference: \x1b[33m$%.2f\x1b[m (\x1b[33m%.2f%%\x1b[m)\n", abs_diff, pct_diff)

	chart.SetXAxis(contract.XAxis())
	f, _ := os.Create("chart.html")
	chart.Render(f)
}

// Pro tips.
//
// To show every tick:
// chart.SetGlobalOptions(charts.WithXAxisOpts(opts.XAxis{
//   AxisLabel: &opts.AxisLabel{Interval: "0"},
// }))
