状态表：

case "forcedDL":
case "metaDL":
case "forcedMetaDL":
case "downloading":
    state = "downloading";
    img_path = "images/downloading.svg";
    break;
case "forcedUP":
case "uploading":
    state = "uploading";
    img_path = "images/upload.svg";
    break;
case "stalledUP":
    state = "stalledUP";
    img_path = "images/stalledUP.svg";
    break;
case "stalledDL":
    state = "stalledDL";
    img_path = "images/stalledDL.svg";
    break;
case "pausedDL":
    state = "torrent-stop";
    img_path = "images/stopped.svg";
    break;
case "pausedUP":
    state = "checked-completed";
    img_path = "images/checked-completed.svg";
    break;
case "queuedDL":
case "queuedUP":
    state = "queued";
    img_path = "images/queued.svg";
    break;
case "checkingDL":
case "checkingUP":
case "queuedForChecking":
case "checkingResumeData":
case "moving":
    state = "force-recheck";
    img_path = "images/force-recheck.svg";
    break;
case "unknown":
case "missingFiles":
    state = "error";
    img_path = "images/error.svg";
    break;


regex:
"hash":"(.*)","infohash_v1"(.*)"name":"(.*)"(.*),"progress":(\d+(\.\d+)?),"(.*)"state":"(.*)","super_seeding"

list:
[{"added_on":1693566863,"amount_left":0,
"auto_tmm":false,"availability":-1,
"category":"","completed":16077317,
"completion_on":1693623513,
"content_path":"C:\\Users\\Wei\\Desktop\\work\\wei-updater\\test\\data\\new\\0.1.2",
"dl_limit":0,"dlspeed":0,"download_path":"",
"downloaded":16697948,"downloaded_session":16697948,
"eta":8640000,"f_l_piece_prio":false,
"force_start":false,"hash":"858d5efde656ff3b75ef5be8204cec505983a4d0",
"infohash_v1":"858d5efde656ff3b75ef5be8204cec505983a4d0","infohash_v2":"",
"last_activity":1693623495,
"max_ratio":-1,"max_seeding_time":-1,"name":"0.1.2",
"num_complete":4,"num_incomplete":0,"num_leechs":0,"num_seeds":0,
"priority":0,"progress":1,"ratio":0,"ratio_limit":-2,
"save_path":"C:\\Users\\Wei\\Desktop\\work\\wei-updater\\test\\data\\new",
"seeding_time":163418,"seeding_time_limit":-2,
"seen_complete":1693623513,"seq_dl":false,"size":16077317,
"state":"stalledUP","super_seeding":false,"tags":"",
"time_active":220073,"total_size":16077317,
"tracker":"udp://tracker.moeking.me:6969/announce",
"trackers_count":143,"up_limit":0,"uploaded":0,
"uploaded_session":0,"upspeed":0}]
