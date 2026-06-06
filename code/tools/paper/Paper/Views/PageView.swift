//
//  PageView.swift
//  Paper
//
//  Created by khang on 8/5/23.
//

import UIKit

class PageView: UIView {
    override var intrinsicContentSize: CGSize {
        PaperSize.a4.frame.scale(by: 4)
    }

    var buffer = Line(color: .blue)

    // MARK: - touches* overrides

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let touch = touches.first {
            touchBegan(at: touch, with: event)
        }
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let touch = touches.first {
            touchMoved(to: touch, with: event)
        }
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let touch = touches.first {
            touchEnded(at: touch, with: event)
        }
    }
    
    // MARK: - draw/render override

    override func draw(_ rect: CGRect) {
        super.draw(rect)

        guard let context = UIGraphicsGetCurrentContext() else {
            return
        }
        
        buffer.loadProps(onto: context)
        buffer.stroke(onto: context)
    }
    
    // MARK: - Custom business logic

    func touchBegan(at touch: UITouch, with event: UIEvent?) {
        let point = touch.location(in: self)
        console.debug("BEGAN: \(point)")

        let render = buffer.start(at: touch.location(in: self))
        setNeedsDisplay(render)
    }

    func touchMoved(to touch: UITouch, with event: UIEvent?) {
        let point = touch.location(in: self)
        console.debug("MOVED: \(point)")

        let render = buffer.move(to: touch.location(in: self))
        setNeedsDisplay(render)
        
    }

    func touchEnded(at touch: UITouch, with event: UIEvent?) {
        let point = touch.location(in: self)
        console.debug("ENDED: \(point)")

        let render = buffer.end(at: touch.location(in: self))
        setNeedsDisplay(render)
    }
}
