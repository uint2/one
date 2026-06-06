//
//  Tailwind.swift
//
//  Created by khang on 8/5/23.
//

import UIKit

/**
 * Extend UIColor to be initialized by a hex code string
 */
extension UIColor {
    public convenience init?(hex: String) {
        if !hex.hasPrefix("#") { return nil }
        let hexColor = String(hex[hex.index(hex.startIndex, offsetBy: 1)...])
        if hexColor.count != 6 { return nil }
        let scanner = Scanner(string: hexColor)
        var hexNumber: UInt64 = 0
        if scanner.scanHexInt64(&hexNumber) {
            let r = CGFloat((hexNumber & 0x00FF_0000) >> 16) / 255
            let g = CGFloat((hexNumber & 0x0000_FF00) >> 8) / 255
            let b = CGFloat(hexNumber & 0x0000_00FF) / 255
            self.init(red: r, green: g, blue: b, alpha: 1)
            return
        }
        return nil
    }
}

enum Tailwind {
    struct Shade {
        let x100, x200, x300, x400, x500, x600, x700, x800, x900: UIColor
        init(_ x: [String]) {
            self.x100 = UIColor(hex: x[0])!
            self.x200 = UIColor(hex: x[1])!
            self.x300 = UIColor(hex: x[2])!
            self.x400 = UIColor(hex: x[3])!
            self.x500 = UIColor(hex: x[4])!
            self.x600 = UIColor(hex: x[5])!
            self.x700 = UIColor(hex: x[6])!
            self.x800 = UIColor(hex: x[7])!
            self.x900 = UIColor(hex: x[8])!
        }
    }

    static var slate: Shade { Shade(["#f1f5f9", "#e2e8f0", "#cbd5e1", "#94a3b8", "#64748b", "#475569", "#334155", "#1e293b", "#0f172a"]) }
    static var gray: Shade { Shade(["#f3f4f6", "#e5e7eb", "#d1d5db", "#9ca3af", "#6b7280", "#4b5563", "#374151", "#1f2937", "#111827"]) }
    static var zinc: Shade { Shade(["#f4f4f5", "#e4e4e7", "#d4d4d8", "#a1a1aa", "#71717a", "#52525b", "#3f3f46", "#27272a", "#18181b"]) }
    static var neutral: Shade { Shade(["#f5f5f5", "#e5e5e5", "#d4d4d4", "#a3a3a3", "#737373", "#525252", "#404040", "#262626", "#171717"]) }
    static var stone: Shade { Shade(["#f5f5f4", "#e7e5e4", "#d6d3d1", "#a8a29e", "#78716c", "#57534e", "#44403c", "#292524", "#1c1917"]) }
    static var red: Shade { Shade(["#fee2e2", "#fecaca", "#fca5a5", "#f87171", "#ef4444", "#dc2626", "#b91c1c", "#991b1b", "#7f1d1d"]) }
    static var orange: Shade { Shade(["#ffedd5", "#fed7aa", "#fdba74", "#fb923c", "#f97316", "#ea580c", "#c2410c", "#9a3412", "#7c2d12"]) }
    static var amber: Shade { Shade(["#fef3c7", "#fde68a", "#fcd34d", "#fbbf24", "#f59e0b", "#d97706", "#b45309", "#92400e", "#78350f"]) }
    static var yellow: Shade { Shade(["#fef9c3", "#fef08a", "#fde047", "#facc15", "#eab308", "#ca8a04", "#a16207", "#854d0e", "#713f12"]) }
    static var lime: Shade { Shade(["#ecfccb", "#d9f99d", "#bef264", "#a3e635", "#84cc16", "#65a30d", "#4d7c0f", "#3f6212", "#365314"]) }
    static var green: Shade { Shade(["#dcfce7", "#bbf7d0", "#86efac", "#4ade80", "#22c55e", "#16a34a", "#15803d", "#166534", "#14532d"]) }
    static var emerald: Shade { Shade(["#d1fae5", "#a7f3d0", "#6ee7b7", "#34d399", "#10b981", "#059669", "#047857", "#065f46", "#064e3b"]) }
    static var teal: Shade { Shade(["#ccfbf1", "#99f6e4", "#5eead4", "#2dd4bf", "#14b8a6", "#0d9488", "#0f766e", "#115e59", "#134e4a"]) }
    static var cyan: Shade { Shade(["#cffafe", "#a5f3fc", "#67e8f9", "#22d3ee", "#06b6d4", "#0891b2", "#0e7490", "#155e75", "#164e63"]) }
    static var sky: Shade { Shade(["#e0f2fe", "#bae6fd", "#7dd3fc", "#38bdf8", "#0ea5e9", "#0284c7", "#0369a1", "#075985", "#0c4a6e"]) }
    static var blue: Shade { Shade(["#dbeafe", "#bfdbfe", "#93c5fd", "#60a5fa", "#3b82f6", "#2563eb", "#1d4ed8", "#1e40af", "#1e3a8a"]) }
    static var indigo: Shade { Shade(["#e0e7ff", "#c7d2fe", "#a5b4fc", "#818cf8", "#6366f1", "#4f46e5", "#4338ca", "#3730a3", "#312e81"]) }
    static var violet: Shade { Shade(["#ede9fe", "#ddd6fe", "#c4b5fd", "#a78bfa", "#8b5cf6", "#7c3aed", "#6d28d9", "#5b21b6", "#4c1d95"]) }
    static var purple: Shade { Shade(["#f3e8ff", "#e9d5ff", "#d8b4fe", "#c084fc", "#a855f7", "#9333ea", "#7e22ce", "#6b21a8", "#581c87"]) }
    static var fuchsia: Shade { Shade(["#fae8ff", "#f5d0fe", "#f0abfc", "#e879f9", "#d946ef", "#c026d3", "#a21caf", "#86198f", "#701a75"]) }
    static var pink: Shade { Shade(["#fce7f3", "#fbcfe8", "#f9a8d4", "#f472b6", "#ec4899", "#db2777", "#be185d", "#9d174d", "#831843"]) }
    static var rose: Shade { Shade(["#ffe4e6", "#fecdd3", "#fda4af", "#fb7185", "#f43f5e", "#e11d48", "#be123c", "#9f1239", "#881337"]) }
}
