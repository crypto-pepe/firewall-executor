# Enable/disable dry run

Enables or disables dry run

**URL** : `/api/admin`

**Method** : `POST`

### Request constraints

**Content-Type**: `application/json`

**Body**:

| Field       | Type     | Is required | Note               |
|-------------|----------|-------------|--------------------|
| `dry_run`   | 'bool`   | False       | Enable dry run mod | 
| `log_level` | 'string` | False       | Sets log level     | 

**Request examples**

```json
{
  "dry_run": true
}
```

```json
{
  "dry_run": false,
  "log_level": "trace"
}
```

## Success Response

**Condition** : Dry run mod was successfully enabled/disabled.

**Code** : `200 OK`

## Error Responses

**Condition** : query is incorrect.

**Code** : `400 BAD REQUEST`