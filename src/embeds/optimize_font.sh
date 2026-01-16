#!/bin/bash
set -e

# This script optimizes the "Inter.ttf" font by removing unused characters (subsetting).
# It significantly reduces the file size (e.g., from ~800KB to ~200KB).
#
# Requirements:
# - Python 3 installed
# - Inter.ttf must be present in the same directory as this script.

echo "Starting font optimization..."

# 1. Create a temporary Python virtual environment (.font_opt_venv)
#    This isolates the dependencies so we don't mess up your system Python.
rm -rf .font_opt_venv
python3 -m venv .font_opt_venv

# 2. Activate the virtual environment
source .font_opt_venv/bin/activate

# 3. Install 'fonttools' library, which contains the 'pyftsubset' tool needed for optimization.
pip install fonttools

# 4. Run the subsetting tool (pyftsubset)
#    Arguments:
#    - Inter.ttf: The source font file (taken as base).
#    - --unicodes: Specifies which characters to KEEP.
#         0020-007F : Basic Latin (English letters A-Z, a-z, numbers, basic punctuation)
#         0400-04FF : Cyrillic (Russian letters and other Cyrillic characters)
#         2000-206F : General Punctuation (special quotes, dashes, etc.)
#    - --output-file: Where to save the optimized font.
# pyftsubset Inter.ttf \
#   --unicodes="0020-007F,0400-04FF,2000-206F" \
#   --output-file=Inter_opt.ttf

# Optimize UbuntuMono
pyftsubset UbuntuMono.ttf \
  --unicodes="0020-007F,0400-04FF,2000-206F" \
  --output-file=UbuntuMono_opt.ttf

# 5. Show the size of the generated file to confirm optimization.
ls -lh Inter_opt.ttf

# 6. Deactivate and clean up the temporary virtual environment.
deactivate
rm -rf .font_opt_venv

echo "Optimization successfully completed. Created Inter_opt.ttf"
