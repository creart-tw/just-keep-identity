#!/bin/bash
# Mock Test Runner for JKI Security Audit

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

TEST_REPO="/tmp/jki-mock-repo"
AUDIT_SCRIPT="$(pwd)/scripts/security-audit.sh"

setup_repo() {
    rm -rf "$TEST_REPO"
    mkdir -p "$TEST_REPO"
    cd "$TEST_REPO"
    git init -q
    echo "target/" > .gitignore
    echo "private/" >> .gitignore
    echo "master.key" >> .gitignore
    echo "vault.json" >> .gitignore
    echo "data/" >> .gitignore
    echo "config.json" >> .gitignore
}

run_test() {
    local name=$1
    echo -e "\nRunning Test: $name"
    cd "$TEST_REPO"
    bash "$AUDIT_SCRIPT" > /dev/null 2>&1
    if [ $? -eq $2 ]; then
        echo -e "${GREEN}[PASS]${NC} Expected result received."
    else
        echo -e "${RED}[FAIL]${NC} Unexpected result!"
        exit 1
    fi
}

# --- SCENARIOS ---

# 1. Normal Secure State
setup_repo
mkdir private && touch private/secret.txt
run_test "Normal Secure State" 0

# 2. Missing Exclusion (The "Phantom Erasure" Incident)
setup_repo
# Simulate Agent's mistake: overwriting .gitignore without 'private/'
echo "target/" > .gitignore
run_test "Missing Critical Exclusion" 1

# 3. Untracked Sensitive File
setup_repo
touch master.key
# Force it to be untracked by removing from gitignore
sed -i '' '/master.key/d' .gitignore 2>/dev/null || sed -i '/master.key/d' .gitignore
run_test "Untracked Sensitive File" 1

# 4. History Leak
setup_repo
mkdir -p private
echo "leak" > private/leak.txt
git add .gitignore
git commit -m "Initial" -q
# Accidentally add a private file to history
git add -f private/leak.txt
git commit -m "Oops" -q
run_test "Historical Leak Detection" 1

echo -e "\n${GREEN}All Mock Scenarios Passed!${NC}"
rm -rf "$TEST_REPO"
