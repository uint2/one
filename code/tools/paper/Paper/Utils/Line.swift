//
//  Line.swift
//  Paper
//
//  Created by khang on 29/5/23.
//

import UIKit

class Line {
    let bezierPath: UIBezierPath
    var color: UIColor
    var previousPoint: CGPoint

    var cgPath: CGPath {
        bezierPath.cgPath
    }
    
    var lineWidth: CGFloat {
        bezierPath.lineWidth
    }

    init(color: UIColor, lineWidth: CGFloat = 2.0) {
        self.color = color
        self.bezierPath = UIBezierPath()
        bezierPath.lineWidth = lineWidth
        self.previousPoint = .zero
    }

    /**
     * Returns the rect that needs to be re-rendered
     */
    func start(at point: CGPoint) -> CGRect {
        bezierPath.move(to: point)
        bezierPath.addLine(to: point)
        previousPoint = point
        return CGRect(x: point.x - lineWidth / 2, y: point.y - lineWidth / 2, width: lineWidth, height: lineWidth)
    }

    /**
     * Returns the rect that needs to be re-rendered
     *
     *          * P2        * P4
     *         ___         /
     *        /   \       /
     *       * M1  * M2  * M3
     *      /       \   /
     *     /         ‾‾‾
     *    * P1        * P3
     *
     * self.bezierPath are essentially a set of midpoints
     */
    func move(to point: CGPoint) -> CGRect {
        let midpoint = previousPoint.midpoint(to: point)
        let render = bezierPath.currentPoint.rectBetween(midpoint, lineWidth: lineWidth)
        bezierPath.addQuadCurve(to: midpoint, controlPoint: previousPoint)
        previousPoint = point
        return render
    }

    /**
     * Returns the rect that needs to be re-rendered
     */
    func end(at point: CGPoint) -> CGRect {
        let render = bezierPath.currentPoint.rectBetween(point, lineWidth: lineWidth)
        bezierPath.addLine(to: point)
        return render
    }

    func stroke(onto context: CGContext) {
        context.addPath(cgPath)
        context.strokePath()
    }

    func loadProps(onto context: CGContext) {
        context.setLineCap(.round)
        context.setLineJoin(.round)
        context.setStrokeColor(color.cgColor)
        context.setLineWidth(lineWidth)
    }
}
