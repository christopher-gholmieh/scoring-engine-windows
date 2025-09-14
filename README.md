# Scoring Engine - Windows
This is a scoring engine meant to replicate the CyberPatriot AFA competition experience. Practice images can easily be created.

## Configuration
To create a configuration for the scoring engine, create a **YAML** file that follows an example in samples/
* Each check can have multiple pass conditions that must succeed in order for it to qualify as remediated.
* If a check has negative points, it will be qualified as a penalty.

## Instructions
After the creation of your **YAML** configuration, encode it into a .dat using the following command:
```bash
python ./configuration-parser.py <path-to-configuration-file>
```

Then, build the release binary!
```bash
cargo build --release
```

Lastly, execute the binary upon startup with the website folder and assets folder in the same directory.
