//
//  ViewController.swift
//  Paper
//
//  Created by khang on 8/5/23.
//

import UIKit

/**
 * Main View Controller of the entire App
 */
class ViewController: UIViewController, UIScrollViewDelegate {
    // views
    @IBOutlet var scrollView: ScrollView!
    @IBOutlet var docView: DocumentView!

    var pageControllers = [PageViewController]()

    var zoomScale: CGFloat { scrollView.zoomScale }
    var docWidth: CGFloat { docView.bounds.width * zoomScale }
    var viewWidth: CGFloat { view.bounds.width }

    ////////////////////////////////////////////////////////////////////////////

    // MARK: - Hook Overrides

    override func viewDidLoad() {
        super.viewDidLoad()

        scrollView.delegate = self
        (scrollView.minimumZoomScale, scrollView.maximumZoomScale) = (1, 10)

        colorForProduction()

        setupPages(count: 3)
        scrollView.centerContentWithInset()
    }

    func viewForZooming(in: UIScrollView) -> UIView? { docView }

    func scrollViewDidZoom(_: UIScrollView) {
        scrollView.centerContentWithInset()
    }

    override func viewDidLayoutSubviews() {
        scrollView.centerContentWithInset()
    }

    ////////////////////////////////////////////////////////////////////////////

    // MARK: - Other functions

    func colorForDebugging() {
        scrollView.colorize(Tailwind.red)
        docView.colorize(Tailwind.emerald)
    }

    func colorForProduction() {
        view.backgroundColor = Theme.bgColor
    }
}
