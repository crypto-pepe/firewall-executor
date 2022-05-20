# Enable/disable dry run

Enables or disables dry run

**URL** : `/api/admin`

**Method** : `POST`

### Request constraints

**Query**:

| Field     | Type   | Is required | Note               |
|-----------|--------|-------------|--------------------|
| `dry_run` | 'bool` | Yes         | Enable dry run mod | 

## Success Response

**Condition** : Dry run mod was successfully enabled/disabled.

**Code** : `200 OK`

## Error Responses

**Condition** : query is incorrect.

**Code** : `400 BAD REQUEST`