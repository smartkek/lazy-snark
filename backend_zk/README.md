# Usage
0. [install](https://zokrates.github.io/gettingstarted.html) ZoKrates

1. compile ```zokrates compile -i root.code```

2. perform the setup phase ```zokrates setup```

3. execute the program ```zokrates compute-witness -a 0 0 0 5 5 0 0 0 263561599766550617289250058199814760685 65303172752238645975888084098459749904 121528245299328017710050549170605934178 329200266467600403224363203181133000487```

4. generate a proof of computation ```zokrates generate-proof```

5. export a solidity verifier ```zokrates export-verifier```
