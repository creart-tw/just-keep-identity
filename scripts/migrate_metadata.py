import json
import yaml
import os

def migrate():
    jki_home = os.environ.get('JKI_HOME', os.path.expanduser('~/.config/jki'))
    old_path = os.path.join(jki_home, 'vault.metadata.json')
    new_path = os.path.join(jki_home, 'vault.metadata.yaml')

    if not os.path.exists(old_path):
        print(f"No JSON metadata found at {old_path}")
        return

    with open(old_path, 'r') as f:
        data = json.load(f)

    with open(new_path, 'w') as f:
        yaml.dump(data, f, sort_keys=False, allow_unicode=True)
    
    print(f"Successfully migrated {old_path} to {new_path}")
    print("You can now safely remove the old .json file.")

if __name__ == "__main__":
    migrate()
