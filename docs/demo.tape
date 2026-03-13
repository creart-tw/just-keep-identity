# JKI VHS Demo Tape
Output docs/assets/demo.gif

Set FontSize 20
Set Width 1200
Set Height 600
Set Padding 40
Set Theme "Catppuccin Frappe"
Set WindowBar Colorful
Set CursorBlink false

# Isolate environment to mock data
Env JKI_HOME "./mock"

# --- Start Scenarios ---

# 1. Check System Status
Type "jkim status"
Sleep 500ms
Enter
Sleep 3s

# 2. List All Accounts
Type "jki"
Sleep 500ms
Enter
Sleep 4s

# 3. High-speed Fuzzy Search with Highlighting (Force List)
# Target: alice.dev@gmail.com [Google]
Type "jki goog -l"
Sleep 800ms
Enter
# Now highlights 'g', 'o', 'o', 'g' in Google
Sleep 5s

# 4. Smart Intelligence (Multiple Matches with Score Gap)
# Target: admin-prod [AWS] vs others
Type "jki ad"
Sleep 800ms
Enter
# Highlights matches and shows score gaps
Sleep 5s

# 5. Explicit Index Selection
Type "jki 4"
Sleep 800ms
Enter
# Instantly picks the 4th item (OpenAI)
Sleep 4s

# 6. Clean up
Type "clear"
Enter
Sleep 1s
Show
