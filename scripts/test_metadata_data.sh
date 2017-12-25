#!/bin/bash

$(./scripts/launch_metadata.sh) &
$(./scripts/launch_data.sh) &

echo "Press any key to exit: ";
read anykey;