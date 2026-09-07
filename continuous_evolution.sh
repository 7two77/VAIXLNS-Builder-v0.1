#!/bin/bash

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║   VAIXLNS CONTINUOUS EVOLUTION — Self-Sustaining Cycle     ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

MAX_ITERATIONS=${1:-5}
CURRENT_ITERATION=0
SUCCESS_COUNT=0
FAIL_COUNT=0

mkdir -p evolution_logs
mkdir -p output

echo "📊 Starting Continuous Evolution Cycle"
echo "   Max iterations: $MAX_ITERATIONS"
echo "   Log directory: evolution_logs/"
echo ""

while [ $CURRENT_ITERATION -lt $MAX_ITERATIONS ]; do
    CURRENT_ITERATION=$((CURRENT_ITERATION + 1))
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🔄 Iteration $CURRENT_ITERATION of $MAX_ITERATIONS"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    LOG_FILE="evolution_logs/iteration_$(printf "%03d" $CURRENT_ITERATION).log"
    OUTPUT_FILE="output/evolved_$(printf "%03d" $CURRENT_ITERATION).rs"
    
    echo "🔨 Building..."
    cargo build --release 2>&1 | tail -5
    
    echo "🧬 Evolving..."
    cargo run --release -- VAIXLNS_ROOT.lns "$OUTPUT_FILE" 2>&1 | tee "$LOG_FILE"
    
    if grep -q "GENESIS COMPLETE" "$LOG_FILE"; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        echo "✅ Iteration $CURRENT_ITERATION: SUCCESSFUL"
        
        ENTITIES=$(grep "Entities:" "$LOG_FILE" | tail -1 | awk '{print $2}')
        RELATIONS=$(grep "Relations:" "$LOG_FILE" | tail -1 | awk '{print $2}')
        PHASES=$(grep "Total phases:" "$LOG_FILE" | tail -1 | awk '{print $3}')
        BYTES=$(grep "bytes" "$LOG_FILE" | tail -1 | awk '{print $6}')
        
        echo "   📊 Entities: ${ENTITIES:-0}"
        echo "   📊 Relations: ${RELATIONS:-0}"
        echo "   📊 Phases: ${PHASES:-0}"
        echo "   📊 Bytes: ${BYTES:-0}"
    else
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "❌ Iteration $CURRENT_ITERATION: FAILED"
    fi
    
    echo ""
    echo "📊 Progress: $CURRENT_ITERATION/$MAX_ITERATIONS"
    echo "   ✅ Successful: $SUCCESS_COUNT"
    echo "   ❌ Failed: $FAIL_COUNT"
    echo ""
    
    sleep 1
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 FINAL REPORT"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Total iterations: $MAX_ITERATIONS"
echo "✅ Successful: $SUCCESS_COUNT"
echo "❌ Failed: $FAIL_COUNT"
echo "📈 Success rate: $(( (SUCCESS_COUNT * 100) / MAX_ITERATIONS ))%"
echo ""
echo "📁 Generated files:"
ls -la output/evolved_*.rs 2>/dev/null || echo "   No files"
echo ""
echo "✅ Continuous Evolution Complete!"
