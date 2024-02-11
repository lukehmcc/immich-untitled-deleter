# Immich Untitled Deleter

### So what does it do?
All this tool does is take your immich URL, API Key, and then delete all albums that are formatted in the regex pattern `Untitled\((?:\d+)\)`. 

### Why?
Google Photos Takeout has some real borked metadata & folder structures. Modern takeout tools like [google-photos-migrate](https://github.com/garzj/google-photos-migrate) can avoid importing bad albums, but many people like me didn't migrate with advanced tools and thus have hundreds of albums cluttering up our space. To fix this I wrote this little tool.

### Usage
Use this with an .env (as shown in the `example/` folder). Put the `.env` file in the same directory as the executable. 

### Build & Install
Download the latest release from the Github page.

Or build yourself:
```bash
git clone https://github.com/lukehmcc/immich-untitled-deleter.git
cd immich-untitled-deleter
cargo build --release
```
