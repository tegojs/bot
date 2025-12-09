import { ExternalLink } from "lucide-react";

export function AboutSettings() {
  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-white">About</h2>

      {/* App Info */}
      <div className="flex items-center gap-4 p-4 bg-gray-800/50 rounded-lg">
        <div className="w-16 h-16 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-2xl font-bold text-white">
          A
        </div>
        <div>
          <h3 className="text-lg font-semibold text-white">Aumate</h3>
          <p className="text-sm text-gray-400">Version 0.1.0</p>
        </div>
      </div>

      {/* Description */}
      <p className="text-sm text-gray-300 leading-relaxed">
        Aumate is a powerful command palette for desktop automation. Quickly
        access commands, run scripts, and control your computer with just a few
        keystrokes.
      </p>

      {/* Links */}
      <div className="space-y-2">
        <LinkButton href="https://github.com/tegojs/bot">
          GitHub Repository
        </LinkButton>
        <LinkButton href="https://github.com/tegojs/bot/issues">
          Report an Issue
        </LinkButton>
        <LinkButton href="https://github.com/tegojs/bot/releases">
          Release Notes
        </LinkButton>
      </div>

      {/* Credits */}
      <div className="pt-4 border-t border-white/10">
        <h3 className="text-sm font-medium text-white mb-3">Credits</h3>
        <p className="text-xs text-gray-400">
          Built with Tauri, React, and Rust.
        </p>
        <p className="text-xs text-gray-400 mt-1">
          Inspired by Raycast and Alfred.
        </p>
      </div>

      {/* Copyright */}
      <div className="pt-4 text-xs text-gray-500">
        <p>&copy; 2024 Aumate. All rights reserved.</p>
      </div>
    </div>
  );
}

interface LinkButtonProps {
  href: string;
  children: React.ReactNode;
}

function LinkButton({ href, children }: LinkButtonProps) {
  return (
    <a
      href={href}
      target="_blank"
      rel="noopener noreferrer"
      className="flex items-center justify-between px-4 py-3 text-sm bg-gray-800 hover:bg-gray-700 rounded-lg text-gray-300 transition-colors group"
    >
      {children}
      <ExternalLink className="w-4 h-4 text-gray-500 group-hover:text-gray-300" />
    </a>
  );
}
