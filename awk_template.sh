#!/bin/bash

cat > $1 << EOF
#!/usr/bin/awk -f

BEGIN {

}

{

}

END {

}
EOF

chmod +x $1
