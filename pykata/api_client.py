"""
Katanaute API Client for PyKata GUI application.
Handles authentication (device flow and token-based) and API requests.
"""

import json
import os
import time
from pathlib import Path
from typing import Optional, Dict, List, Any
from datetime import datetime

import requests


class KatanauteAPIError(Exception):
    """Custom exception for API errors."""
    pass


class APIClient:
    """Client for interacting with the Katanaute Phoenix backend API."""

    def __init__(self, base_url: str = "http://localhost:4000/api"):
        self.base_url = base_url.rstrip('/')
        self.token: Optional[str] = None
        self.user: Optional[Dict[str, Any]] = None
        self.token_file = Path.home() / '.pykata_token'

        # Try to load saved token
        self._load_token()

    def _load_token(self) -> bool:
        """Load saved token from file."""
        try:
            if self.token_file.exists():
                with open(self.token_file, 'r') as f:
                    data = json.load(f)
                    self.token = data.get('token')
                    self.user = data.get('user')
                    return True
        except Exception as e:
            print(f"Error loading token: {e}")
        return False

    def _save_token(self):
        """Save token to file."""
        try:
            with open(self.token_file, 'w') as f:
                json.dump({
                    'token': self.token,
                    'user': self.user
                }, f)
            # Set restrictive permissions (Unix only)
            try:
                os.chmod(self.token_file, 0o600)
            except:
                pass
        except Exception as e:
            print(f"Error saving token: {e}")

    def _clear_token(self):
        """Clear saved token."""
        self.token = None
        self.user = None
        try:
            if self.token_file.exists():
                self.token_file.unlink()
        except Exception as e:
            print(f"Error clearing token: {e}")

    def _headers(self) -> Dict[str, str]:
        """Get headers for authenticated requests."""
        headers = {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        }
        if self.token:
            headers['Authorization'] = f'Bearer {self.token}'
        return headers

    def is_authenticated(self) -> bool:
        """Check if user is authenticated."""
        return self.token is not None

    # Authentication methods

    def register(self, email: str, password: str) -> Dict[str, Any]:
        """Register a new user."""
        url = f"{self.base_url}/auth/register"
        data = {'email': email, 'password': password}

        try:
            response = requests.post(url, json=data, headers=self._headers())
            response.raise_for_status()
            result = response.json()

            # Extract token and user info
            self.token = result['data']['access_token']
            self.user = result['data']['user']
            self._save_token()

            return result['data']
        except requests.exceptions.RequestException as e:
            if hasattr(e, 'response') and e.response is not None:
                try:
                    error_data = e.response.json()
                    raise KatanauteAPIError(f"Registration failed: {error_data.get('errors', str(e))}")
                except:
                    raise KatanauteAPIError(f"Registration failed: {e}")
            raise KatanauteAPIError(f"Registration failed: {e}")

    def login(self, email: str, password: str) -> Dict[str, Any]:
        """Login with email and password."""
        url = f"{self.base_url}/auth/token"
        data = {'email': email, 'password': password}

        try:
            response = requests.post(url, json=data, headers=self._headers())
            response.raise_for_status()
            result = response.json()

            # Extract token and user info
            self.token = result['data']['access_token']
            self.user = result['data']['user']
            self._save_token()

            return result['data']
        except requests.exceptions.RequestException as e:
            if hasattr(e, 'response') and e.response is not None:
                try:
                    error_data = e.response.json()
                    raise KatanauteAPIError(f"Login failed: {error_data.get('errors', str(e))}")
                except:
                    raise KatanauteAPIError(f"Login failed: {e}")
            raise KatanauteAPIError(f"Login failed: {e}")

    def logout(self):
        """Logout and revoke token."""
        if not self.token:
            return

        try:
            url = f"{self.base_url}/auth/token"
            requests.delete(url, headers=self._headers())
        except:
            pass  # Even if server request fails, clear local token
        finally:
            self._clear_token()

    def get_current_user(self) -> Dict[str, Any]:
        """Get current authenticated user info."""
        url = f"{self.base_url}/auth/me"

        try:
            response = requests.get(url, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            self.user = result['data']
            return result['data']
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to get user info: {e}")

    # Device flow methods

    def start_device_flow(self) -> Dict[str, Any]:
        """Initiate device authorization flow."""
        url = f"{self.base_url}/auth/device/code"

        try:
            response = requests.post(url, headers=self._headers())
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to start device flow: {e}")

    def poll_device_token(self, device_code: str, interval: int = 5, timeout: int = 300) -> bool:
        """
        Poll for device authorization completion.
        Returns True if successful, False if denied or timed out.
        """
        url = f"{self.base_url}/auth/device/token"
        data = {'device_code': device_code}
        start_time = time.time()

        while time.time() - start_time < timeout:
            try:
                response = requests.post(url, json=data, headers=self._headers())

                if response.status_code == 200:
                    # Authorization successful
                    result = response.json()
                    self.token = result['data']['access_token']
                    self.user = result['data']['user']
                    self._save_token()
                    return True

                # Check for error responses
                error_data = response.json()
                error = error_data.get('errors', {})

                if isinstance(error, dict):
                    error_code = error.get('code', '')
                    if error_code == 'access_denied':
                        return False
                    elif error_code == 'expired_token':
                        return False
                    # authorization_pending - continue polling

            except requests.exceptions.RequestException as e:
                print(f"Polling error: {e}")

            time.sleep(interval)

        return False  # Timeout

    # Session API methods

    def list_sessions(self) -> List[Dict[str, Any]]:
        """Get all sessions."""
        url = f"{self.base_url}/sessions"

        try:
            response = requests.get(url, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            return result['data']
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to list sessions: {e}")

    def get_session(self, session_id: int) -> Dict[str, Any]:
        """Get a specific session."""
        url = f"{self.base_url}/sessions/{session_id}"

        try:
            response = requests.get(url, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            return result['data']
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to get session: {e}")

    def create_session(self, kata_id: int, practiced_at: str,
                      in_course: bool, notes: str = "") -> Dict[str, Any]:
        """Create a new session."""
        url = f"{self.base_url}/sessions"
        data = {
            'session': {
                'kata_id': kata_id,
                'practiced_at': practiced_at,
                'in_course': in_course,
                'notes': notes
            }
        }

        try:
            response = requests.post(url, json=data, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            return result['data']
        except requests.exceptions.RequestException as e:
            if hasattr(e, 'response') and e.response is not None:
                try:
                    error_data = e.response.json()
                    raise KatanauteAPIError(f"Failed to create session: {error_data.get('errors', str(e))}")
                except:
                    raise KatanauteAPIError(f"Failed to create session: {e}")
            raise KatanauteAPIError(f"Failed to create session: {e}")

    def update_session(self, session_id: int, kata_id: int, practiced_at: str,
                      in_course: bool, notes: str = "") -> Dict[str, Any]:
        """Update an existing session."""
        url = f"{self.base_url}/sessions/{session_id}"
        data = {
            'session': {
                'kata_id': kata_id,
                'practiced_at': practiced_at,
                'in_course': in_course,
                'notes': notes
            }
        }

        try:
            response = requests.put(url, json=data, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            return result['data']
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to update session: {e}")

    def delete_session(self, session_id: int):
        """Delete a session."""
        url = f"{self.base_url}/sessions/{session_id}"

        try:
            response = requests.delete(url, headers=self._headers())
            response.raise_for_status()
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to delete session: {e}")

    # Kata API methods

    def list_katas(self) -> List[Dict[str, Any]]:
        """Get all katas."""
        url = f"{self.base_url}/katas"

        try:
            response = requests.get(url, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            return result['data']
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to list katas: {e}")

    def get_kata(self, kata_id: int) -> Dict[str, Any]:
        """Get a specific kata."""
        url = f"{self.base_url}/katas/{kata_id}"

        try:
            response = requests.get(url, headers=self._headers())
            response.raise_for_status()
            result = response.json()
            return result['data']
        except requests.exceptions.RequestException as e:
            raise KatanauteAPIError(f"Failed to get kata: {e}")
