#!/bin/bash

# NetRunner CLI - Animation Showcase Demo
# This script demonstrates all the amazing animations and features

clear
echo "🌟 Welcome to NetRunner CLI Animation Showcase! 🌟"
echo "=================================================="
echo
echo "This demo will showcase all the cyberpunk animations and features."
echo "Press Enter to continue..."
read

clear
echo "🎮 Demo 1: Animation Showcase Mode"
echo "=================================="
echo "Demonstrating all spinner types and special effects..."
echo
echo "5" | timeout 30s ./target/release/netrunner_cli || true
echo
echo "Press Enter to continue to next demo..."
read

clear
echo "🚀 Demo 2: Speed Test with New Animations"
echo "========================================="
echo "Watch the Pacman, download/upload spinners, and ping animations!"
echo
timeout 45s ./target/release/netrunner_cli --mode speed || true
echo
echo "Press Enter to continue to next demo..."
read

clear
echo "🔍 Demo 3: Network Diagnostics with DNA Helix & Rocket"
echo "======================================================="
echo "See the DNA helix, rocket spinners, and matrix effects!"
echo
timeout 60s ./target/release/netrunner_cli --mode diag || true
echo
echo "Press Enter to continue to next demo..."
read

clear
echo "🛠️ Demo 4: Server Analysis with Cyberpunk Scanners"
echo "==================================================="
echo "Advanced server testing with detailed analysis..."
echo
timeout 30s ./target/release/netrunner_cli --mode servers --debug-servers || true
echo
echo "Press Enter to continue to final demo..."
read

clear
echo "🌐 Demo 5: Full Network Analysis"
echo "================================"
echo "Complete analysis combining all animations!"
echo
timeout 90s ./target/release/netrunner_cli --mode full || true

clear
echo
echo "✨ Animation Showcase Complete! ✨"
echo "=================================="
echo
echo "🎯 Features demonstrated:"
echo "  • Cyberpunk spinners with custom patterns"
echo "  • Pacman-style data consumption animation"
echo "  • Download/Upload arrows with rocket completion"
echo "  • Ping pong ball bouncing animation"
echo "  • DNA helix for DNS analysis"
echo "  • Rocket boost for speed tests"
echo "  • Wave frequency patterns"
echo "  • Matrix rain effects"
echo "  • Typing and pulse animations"
echo "  • Advanced server selection with Cloudflare backup"
echo "  • Geographic optimization"
echo "  • Enhanced progress tracking"
echo
echo "🌟 NetRunner CLI - Your cyberpunk internet analyzer! 🌟"
echo
echo "Try these commands:"
echo "  ./target/release/netrunner_cli --mode speed"
echo "  ./target/release/netrunner_cli --mode diag"
echo "  ./target/release/netrunner_cli --mode servers --debug-servers"
echo "  ./target/release/netrunner_cli # Interactive mode"