//
//  ViewController+.swift
//  Paper
//
//  Created by khang on 12/5/23.
//

import UIKit

/**
 * Debugging extension to add pages
 */
extension ViewController {
    func setupPages(count: Int) {
        for _ in 1...count {
            let controller = PageViewController()
            if let page = controller.pageView {
                docView.addArrangedSubview(page)
            }
            addChild(controller)
            pageControllers.append(controller)
        }
        view.layoutIfNeeded()
    }
}
