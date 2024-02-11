# Immich Untitled Deleter

### So what does it do?
All this tool does is take your immich URL, API Key, and then delete all albums that are formatted in the regex pattern `Untitled\((?:\d+|[a-zA-Z]+)\)`. 

### Why?
Google Photos Takeout has some real borked metadata & folder structures. Modern takeout tools like [google-photos-migrate](https://github.com/garzj/google-photos-migrate) can avoid importing bad albums, but many people like me didn't migrate with advanced tools and thus have hundreds of albums cluttering up our space. To fix this I wrote this little tool.

### Build & Install
...