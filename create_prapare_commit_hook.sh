cat > .git/hooks/prepare-commit-msg << EOF
#!/bin/bash

echo "# Advent of code!" >> \$1
D=`echo \$OLDPWD | awk 'BEGIN{FS="/"}{print $NF}'`
sed -i "1i\$D: " \$1
EOF
chmod +x .git/hooks/prepare-commit-msg
