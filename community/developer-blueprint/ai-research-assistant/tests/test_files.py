import requests
import os
from datetime import datetime
import ujson as json

# Set your job_id here
JOB_ID = "1730947535"


def test_transcript():
    base_url = os.getenv("API_SERVICE_URL", "http://localhost:8002")
    print(f"\n[{datetime.now().strftime('%H:%M:%S')}] Testing transcript endpoint...")
    print(f"Job ID: {JOB_ID}")

    try:
        response = requests.get(f"{base_url}/saved_podcast/{JOB_ID}/transcript")

        if response.status_code == 200:
            transcript = response.json()
            print(
                f"[{datetime.now().strftime('%H:%M:%S')}] Successfully retrieved transcript"
            )
            print("\nTranscript content:")
            print(json.dumps(transcript, indent=2))
        else:
            print(
                f"[{datetime.now().strftime('%H:%M:%S')}] Error: {response.status_code}"
            )
            print(f"Response: {response.text}")

    except Exception as e:
        print(f"[{datetime.now().strftime('%H:%M:%S')}] Error: {str(e)}")


def test_prompt_tracker():
    base_url = os.getenv("API_SERVICE_URL", "http://localhost:8002")

    print(f"\n[{datetime.now().strftime('%H:%M:%S')}] Testing history endpoint...")
    print(f"Job ID: {JOB_ID}")

    try:
        response = requests.get(f"{base_url}/saved_podcast/{JOB_ID}/history")

        if response.status_code == 200:
            transcript = response.json()
            print(
                f"[{datetime.now().strftime('%H:%M:%S')}] Successfully retrieved history"
            )
            print("\nHistory content:")
            print(json.dumps(transcript, indent=2))
        else:
            print(
                f"[{datetime.now().strftime('%H:%M:%S')}] Error: {response.status_code}"
            )
            print(f"Response: {response.text}")

    except Exception as e:
        print(f"[{datetime.now().strftime('%H:%M:%S')}] Error: {str(e)}")


if __name__ == "__main__":
    test_transcript()
    test_prompt_tracker()