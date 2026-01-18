#!/bin/bash
set -e

# This script optimizes the "Inter.ttf" font by removing unused characters (subsetting).
# It significantly reduces the file size (e.g., from ~800KB to ~200KB).
#
# Requirements:
# - Python 3 installed
# - Inter.ttf must be present in the same directory as this script.

echo "Starting font optimization..."

# Create a temporary Python virtual environment (.font_opt_venv)
#    This isolates the dependencies so we don't mess up your system Python.
rm -rf .font_opt_venv
python3 -m venv .font_opt_venv

# Activate the virtual environment
source .font_opt_venv/bin/activate

# Install 'fonttools' library, which contains the 'pyftsubset' tool needed for optimization.
pip install fonttools

OUTPUT_DIR="."

if [ -f "Inter.ttf" ]; then
    echo "Optimizing Inter..."
    pyftsubset Inter.ttf \
      --unicodes="0020-007F,0400-04FF,2000-206F" \
      --no-hinting \
      --layout-features="" \
      --drop-tables+=fvar,gvar,HVAR,MVAR,STAT,avar,cvar,DSIG,hdmx,LTSH,VDMX \
      --output-file="$OUTPUT_DIR/Inter_opt.ttf"
else
    echo "Inter.ttf source not found, skipping optimization."
fi

# Optimize UbuntuMono
if [ -f "UbuntuMono.ttf" ]; then
    echo "Optimizing UbuntuMono..."
    pyftsubset UbuntuMono.ttf \
      --unicodes="0020-007F,0400-04FF,2000-206F" \
      --no-hinting \
      --layout-features="" \
      --drop-tables+=fvar,gvar,HVAR,MVAR,STAT,avar,cvar,DSIG,hdmx,LTSH,VDMX \
      --output-file="$OUTPUT_DIR/UbuntuMono_opt.ttf"
else
    echo "UbuntuMono.ttf source not found, skipping optimization."
fi

# Show the size of the generated file to confirm optimization.
ls -lh *_opt.ttf

# Deactivate and clean up the temporary virtual environment.
deactivate
rm -rf .font_opt_venv

echo "Optimization successfully completed. Created Inter_opt.ttf"
