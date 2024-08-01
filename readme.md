# How to
1. Fill `tracks.json` with `[{"title":"...", "artist": "..."}]` contract
2. Create an app on https://developer.spotify.com
   1. Fill Redirect URI. For example: http://localhost:8888 — real server is not needed, any localhost value will work
3. Put this URL to the browser: https://accounts.spotify.com/en/authorize?response_type=code&client_id=$client_id&redirect_uri=$redirect_uri&scope=playlist-read-private%20playlist-read-collaborative%20playlist-modify-public%20playlist-modify-private
   1. Specify `$client_id` and `$redirect_uri` with yours values
   2. You'll see approve window in the browser
   3. Once you approved Spotify will redirect you to the `redirect_uri`. Take the auth code from the address line `?code=<value>`
4. Create a playlist in Spotify UI and copy link to it (share)
   1. Extract id. For example: `https://open.spotify.com/playlist/51ueM8sQllaM5b5wjgOcvP?si=e1845a4a66ba43ad` -> `51ueM8sQllaM5b5wjgOcvP` (after last slash `/`, but before question mark `?`)
5. Fill `.env` with obtained values
6. Run and wait
7. Not found tracks will be added to the `not_found_tracks.json` in the end