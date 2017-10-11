from apistar.test import TestClient
from app import app, welcome


def test_welcome():
    """
    Testing a view directly.
    """
    data = welcome()
    assert data == {'message': 'Welcome to API Star!'}
