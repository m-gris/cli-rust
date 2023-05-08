# aka. SHEBANG => tells the OS to use the env. to execute bash for code belwo
#!/user/bin/env bash

OUTDIR="tests/expected"

# test if OUTDIR does not already exist and creates it
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello   there" > $OUTDIR/hello1.n.txt
echo -n "HEllo" "there" > $OUTDIR/hello2.n.txt
