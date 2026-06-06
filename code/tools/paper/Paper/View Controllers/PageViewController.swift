//
//  PageViewController.swift
//  Bash
//
//  Created by khang on 8/5/23.
//

import UIKit

class PageViewController: UIViewController {
    var pageView: PageView? { self.view as? PageView }
    /**
     * Initialize with a custom view instead of just UIView()
     */
    override init(nibName nibNameOrNil: String?, bundle nibBundleOrNil: Bundle?) {
        super.init(nibName: nibNameOrNil, bundle: nibBundleOrNil)
        self.view = PageView()
        view.colorize(Tailwind.neutral, width: 2)
    }

    // base requirement for overriding `init`
    required init?(coder: NSCoder) { super.init(coder: coder) }

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
    }
}
