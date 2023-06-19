import os
from googleapiclient.discovery import build
from googleapiclient.http import MediaFileUpload
import json
from google_auth_oauthlib.flow import InstalledAppFlow
YOUTUBE = "youtube"
VERSION = "v3"
SCOPES = ['https://www.googleapis.com/auth/youtube.upload']
class Options:
    def __init__(self,title:str,desc:str,keywords:str,category:str,privacy_status:str,file:str,for_kids=False):
        tags = None
        if keywords is not None:
            tags = keywords.split(",")
        self.title = title
        self.desc = desc
        self.tags = tags
        self.category = category
        self.privacy_status = privacy_status
        self.file = file
        self.for_kids = for_kids

class InteractionArgs:
    def __init__(self,id:str,option:str):
        self.id = id
        self.option = option

def client_secret_path(path):
    with open(path,"r") as f:
        return client_from_str(str(f))

def client_secret_env_var(env_name):
    data = os.environ.get(env_name);
    if data is not None:
        return client_from_str(data)
    return None
def set_thumbnail(yt,id:str,file:str):
    yt.thumbnails().set(
        videoId=id,
        media_body=file
    ).execute()

def client_from_str(data):
    data = json.loads(data)
    flow = InstalledAppFlow.from_client_config(data,SCOPES);
    cred = None
    try:
        cred = flow.run_console()
    except:
        pass
    print("client success")
    return build(YOUTUBE,VERSION,credentials=cred)

def upload_req(yt,options:Options):
    body = dict(
        snippet=dict(
            title=options.title,
            description=options.desc,
            tags=options.tags,
            categoryId=options.category
        ),
        status=dict(
            privacyStatus=options.privacy_status,
            selfDeclaredMadeForKids=options.for_kids
        )
    )
    req = yt.videos().insert(
            part= ','.join(body.keys()),
            body=body,
            media_body=MediaFileUpload(options.file, chunksize=-1, resumable=True)
    )
    upload_video(req)

def interact_with_video(yt,args:InteractionArgs):
    yt.videos().rate(
        id=args.id,
        rating=args.option
    ).execute()

# True means it was sucessful and None means it wasn't
def upload_video(request):
    res= None;
    status = None;
    try:
        (status,res) = request.next_chunk();
    except Exception as e:
        print(e)
        return e
    finally:
        print("video was successfully uploaded")
        print("status was %s",status)
        if res is not None:
            if "id" in res:
                print(f"video uploaded: https://www.youtube.com/watch?v={res['id']}")
                pass
            pass
    return None

def main():
    pass
if __name__ == "__main__":
    main()

