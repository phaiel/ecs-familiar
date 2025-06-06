from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    """Manages application settings and secrets."""
    model_config = SettingsConfigDict(env_file='.env', env_file_encoding='utf-8')

    api_key: str = "your-default-api-key"

settings = Settings() 