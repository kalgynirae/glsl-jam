circle() {
  local name=$1
  local color=$2
  local extra=$3
  convert -size 24x24 xc:transparent -fill "$color" -stroke black -draw "$extra circle 12,12 20,20" "png32:$name.png"
}

circle red "#b44738"
circle yellow "#a79026"
circle green "#518921"
circle cyan "#008f89"
circle blue "#3982ce"
circle outline none "stroke-dasharray 2,3"
