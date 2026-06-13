//
//  Extensions.swift
//  Bash
//
//  Created by khang on 8/5/23.
//

import UIKit

extension CGSize {
    func scale(by x: CGFloat) -> CGSize {
        CGSize(width: width * x, height: height * x)
    }
}

extension CGPoint {
    func midpoint(to point: CGPoint) -> CGPoint {
        CGPoint(x: (x + point.x) / 2, y: (y + point.y) / 2)
    }

    func rectBetween(_ p: CGPoint, lineWidth: CGFloat) -> CGRect {
        let H = lineWidth / 2
        let (x, w) = x < p.x ? (x - H, p.x - x + lineWidth) : (p.x - H, x - p.x + lineWidth)
        let (y, h) = y < p.y ? (y - H, p.y - y + lineWidth) : (p.y - H, y - p.y + lineWidth)
        return CGRect(x: x, y: y, width: w, height: h)
    }
}