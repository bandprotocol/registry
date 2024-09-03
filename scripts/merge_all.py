import json
import os
import sys
import yaml

from collections import defaultdict, OrderedDict


def main(input_path: str, output_path: str):
    shared_prefix_traits: dict[str, dict] = {}

    iter_ = os.walk(input_path)
    root, dirs, files = iter_.__next__()
    for d in dirs:
        if (file := f"{d}.yaml") in files:
            with open(os.path.join(root, file), 'r') as f:
                shared_prefix_traits[d] = yaml.safe_load(f)
        else:
            print(f"Missing {d}.yaml")
            exit(1)

    for root, _, files in iter_:
        signal_ids: dict[str, OrderedDict] = defaultdict(OrderedDict)
        prefix = root.split('/')[-1]
        for file in sorted(files):
            with open(os.path.join(root, file), 'r') as f:
                signal_id = file.split('.')[0]
                prefixed_signal_id = f"{prefix}_{signal_id}"

                signal_ids[prefixed_signal_id].update(yaml.safe_load(f))
                signal_ids[prefixed_signal_id].update(shared_prefix_traits[prefix])

        with open(f"{output_path}/registry.json", 'w') as f:
            json.dump(signal_ids, f, indent=4)


if __name__ == '__main__':
    main(*sys.argv[1:])
