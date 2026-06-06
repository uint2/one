//
//  UIView+.swift
//  Paper
//
//  Created by khang on 12/5/23.
//

import UIKit

extension UIView {
    func colorize(_ color: Tailwind.Shade, width: CGFloat = 5) {
        layer.borderColor = color.x400.cgColor
        layer.borderWidth = width
        backgroundColor = color.x200
    }
}
