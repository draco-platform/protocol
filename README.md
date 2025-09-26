# Draco Protocol
## Local Development
To make local development easier, we have created several sh files that should help you get speed. Run them in the following order.

- `sh local_kill_validator.sh`: Kills local validator in case is running.
- `sh local_run_validator.sh`. Runs fresh local validator.
- `sh local_set_up.sh`. Creates DRACO token on the local net, copies its address to the .env file, builds the rust program, and deploys and tests the smart contrat. 

In case any of the last two operations fails, you can run:

- `anchor build` to build the rust program.
- `anchor test --skip-local-validator` to deploy the smart contract to the local net.

Once that is done, you should be settled to run Draco Platform locally!