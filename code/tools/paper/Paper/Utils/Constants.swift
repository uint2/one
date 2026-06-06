//
//  Constants.swift
//  Bash
//
//  Created by khang on 8/5/23.
//

import Foundation

/**
 * Paper sizes. Current suuports (A? paper, B? paper)
 */
enum PaperSize {
    case a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10
    case b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10

    /**
     * Portrait-oriented dimensions in mm
     */
    var frame: CGSize {
        switch self {
        case .a0: return CGSize(width: 841, height: 1189)
        case .a1: return CGSize(width: 594, height: 841)
        case .a2: return CGSize(width: 420, height: 594)
        case .a3: return CGSize(width: 297, height: 420)
        case .a4: return CGSize(width: 210, height: 297)
        case .a5: return CGSize(width: 148.5, height: 210)
        case .a6: return CGSize(width: 105, height: 148.5)
        case .a7: return CGSize(width: 74, height: 105)
        case .a8: return CGSize(width: 52, height: 74)
        case .a9: return CGSize(width: 37, height: 52)
        case .a10: return CGSize(width: 26, height: 37)
        case .b0: return CGSize(width: 1000, height: 1414)
        case .b1: return CGSize(width: 707, height: 1000)
        case .b2: return CGSize(width: 500, height: 707)
        case .b3: return CGSize(width: 353, height: 500)
        case .b4: return CGSize(width: 250, height: 353)
        case .b5: return CGSize(width: 176, height: 250)
        case .b6: return CGSize(width: 125, height: 176)
        case .b7: return CGSize(width: 88, height: 125)
        case .b8: return CGSize(width: 62, height: 88)
        case .b9: return CGSize(width: 44, height: 62)
        case .b10: return CGSize(width: 31, height: 44)
        }
    }
}

func loremIpsum() -> String {
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
}
