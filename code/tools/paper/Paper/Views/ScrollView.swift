//
//  ScrollView.swift
//  Paper
//
//  Created by khang on 12/5/23.
//

import UIKit

class ScrollView: UIScrollView {
    func centerContentWithInset() {
        let x = max(bounds.width - contentSize.width, 0) / 2
        (contentInset.left, contentInset.right) = (x, x)
    }
}
