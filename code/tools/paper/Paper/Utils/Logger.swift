//
//  Logger.swift
//  Paper
//
//  Created by khang on 12/5/23.
//

import Foundation

class Logger {
    private var enabled = true
    private var level: Level = .info
    let dateFormatter = DateFormatter()

    enum Level: Int, CustomStringConvertible {
        case debug
        case info

        var description: String {
            switch self {
            case .info: return "[INFO]"
            case .debug: return "[DEBUG]"
            }
        }
    }

    init() {
        dateFormatter.dateFormat = "HH:mm:ss.SSS"
    }

    private func date() -> String {
        dateFormatter.string(from: Date())
    }

    func enable() {
        enabled = true
    }

    func disable() {
        enabled = false
    }

    func setLevel(_ level: Level) {
        self.level = level
    }

    private func _print(_ thing: Any, level: Level, once: Bool = false) {
        if enabled && level.rawValue >= self.level.rawValue {
            print(date(), level.description, thing)
        }
    }

    func debug(_ thing: Any) {
        _print(thing, level: .debug)
    }

    func info(_ thing: Any) {
        _print(thing, level: .info)
    }

    func log(_ thing: Any) {
        _print(thing, level: level)
    }
}

let console = Logger()
