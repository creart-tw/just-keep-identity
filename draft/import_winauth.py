# /// script
# dependencies = [
#   "urllib3",
#   "typer",
# ]
# ///
import json
import urllib.parse
import os
import uuid
import typer
from typing import Optional

app = typer.Typer()

def parse_otpauth(uri):
    if not uri.startswith("otpauth://"):
        return None
    
    parsed = urllib.parse.urlparse(uri)
    path = parsed.path.strip("/")
    
    if ":" in path:
        issuer, name = path.split(":", 1)
    else:
        issuer, name = None, path
    
    query = urllib.parse.parse_qs(parsed.query)
    name = urllib.parse.unquote(name).replace("+", " ")
    
    secret = query.get("secret", [None])[0]
    if not secret:
        return None
        
    digits = int(query.get("digits", [6])[0])
    issuer_query = query.get("issuer", [None])[0]
    
    effective_issuer = (issuer or issuer_query or "").lower()
    account_type = "Standard"
    if "steam" in effective_issuer:
        account_type = "Steam"
    elif "battle" in effective_issuer or "blizzard" in effective_issuer:
        account_type = "Blizzard"
        
    return {
        "id": str(uuid.uuid4()),
        "name": name,
        "issuer": issuer or issuer_query,
        "secret": secret,
        "digits": digits,
        "algorithm": "SHA1",
        "account_type": account_type
    }

@app.command()
def import_winauth(
    overwrite: bool = typer.Option(False, "--overwrite", help="Update existing accounts if secret matches"),
    txt_path: str = "data/private/winauth-2026-02-19.txt",
    base_output: str = "data/private/",
):
    if not os.path.exists(txt_path):
        print(f"Error: {txt_path} not found.")
        return

    meta_path = os.path.join(base_output, "vault.metadata.json")
    sec_path = os.path.join(base_output, "vault.secrets.json")

    # Load existing metadata to detect conflicts
    existing_meta = []
    if os.path.exists(meta_path):
        with open(meta_path, "r", encoding="utf-8") as f:
            existing_meta = json.load(f).get("accounts", [])
    
    # Load existing secrets
    existing_secs = {}
    if os.path.exists(sec_path):
        with open(sec_path, "r", encoding="utf-8") as f:
            existing_secs = json.load(f) # Map: id -> secret_data

    # Map for deduplication: secret -> account_id
    secret_to_id = {s_data["secret"].strip().upper(): aid for aid, s_data in existing_secs.items()}
    # Map for metadata lookup: id -> index
    id_to_meta_idx = {m["id"]: i for i, m in enumerate(existing_meta)}

    new_count = 0
    updated_count = 0
    skipped_count = 0

    with open(txt_path, 'r', encoding='utf-8') as f:
        for line in f:
            line_str = line.strip()
            if not line_str.startswith("otpauth://"):
                continue
                
            acc = parse_otpauth(line_str)
            if not acc:
                continue

            sec_key = acc["secret"].strip().upper()
            
            if sec_key in secret_to_id:
                aid = secret_to_id[sec_key]
                meta_idx = id_to_meta_idx.get(aid)
                
                if meta_idx is not None:
                    existing_m = existing_meta[meta_idx]
                    is_changed = (
                        acc["name"] != existing_m["name"] or
                        acc["issuer"] != existing_m["issuer"] or
                        acc["digits"] != existing_secs[aid]["digits"]
                    )

                    if is_changed and overwrite:
                        # Update Metadata
                        existing_meta[meta_idx]["name"] = acc["name"]
                        existing_meta[meta_idx]["issuer"] = acc["issuer"]
                        # Update Secret data
                        existing_secs[aid]["digits"] = acc["digits"]
                        updated_count += 1
                    else:
                        skipped_count += 1
            else:
                # Totally new entry
                aid = acc["id"]
                # Metadata (Public-ish)
                existing_meta.append({
                    "id": aid,
                    "name": acc["name"],
                    "issuer": acc["issuer"],
                    "account_type": acc["account_type"]
                })
                # Secret Data (Private)
                existing_secs[aid] = {
                    "secret": acc["secret"],
                    "digits": acc["digits"],
                    "algorithm": acc["algorithm"]
                }
                secret_to_id[sec_key] = aid
                new_count += 1
    
    # Save Metadata
    with open(meta_path, "w", encoding="utf-8") as f:
        json.dump({"accounts": existing_meta, "version": 1}, f, indent=4, ensure_ascii=False)
    
    # Save Secrets
    with open(sec_path, "w", encoding="utf-8") as f:
        json.dump(existing_secs, f, indent=4, ensure_ascii=False)
        
    print(f"Split Import Summary:")
    print(f"  - Metadata: {meta_path}")
    print(f"  - Secrets:  {sec_path}")
    print(f"  - New: {new_count}, Updated: {updated_count}, Skipped: {skipped_count}")

if __name__ == "__main__":
    app()
