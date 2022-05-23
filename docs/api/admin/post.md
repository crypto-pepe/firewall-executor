# Enable/disable dry run

Enables or disables dry run

**URL** : `/api/admin`

**Method** : `POST`

### Request constraints

**Content-Type**: `application/json`

**Body**:

| Field       | Type     | Is required | Note                |
|-------------|----------|-------------|---------------------|
| `dry_run`   | 'bool`   | False       | Enable dry run mode | 
| `log_level` | 'string` | False       | Sets log level      | 

**Request examples**

```json
{
  "dry_run": true
}
```

```json
{
  "dry_run": false,
  "log_level": "error,firewall_executor=trace"
}
```

## Success Response

**Condition** : Dry run mode was enabled/disabled.

**Code** : `200 OK`

## Error Responses

**Condition** : request is incorrect.

**Code** : `400 BAD REQUEST`