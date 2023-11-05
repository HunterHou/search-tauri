use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::sync::Mutex;

use super::const_param::{STATIC_SETTING, STATIC_SETTING_PATH};
use super::model_setting::Setting;

pub fn refresh_setting(setting: &Setting) {
    // 将请求的对象复制到静态设置的对象中
    let new_mutx = Mutex::new(setting.clone());
    STATIC_SETTING.deref().clone_from(&&new_mutx);
    let setting_json = serde_json::to_string(setting).unwrap();
    // 以只写模式打开文件，返回 `io::Result<File>`
    let binding = STATIC_SETTING_PATH;
    let path = Path::new(&binding);
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {:?}: {:?}", &STATIC_SETTING_PATH, why),
        Ok(file) => file,
    };
    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(setting_json.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {:?}: {:?}", &STATIC_SETTING_PATH, why)
        }
        Ok(_) => println!("successfully wrote to {}", setting_json),
    }
}

/**
 * 加载配置文件
 */
pub fn loading_file() -> Setting {
    let mut setting_json = String::new();

    let binding = STATIC_SETTING_PATH;
    let path = Path::new(&binding);
    // 以只读方式打开路径，返回 `io::Result<File>`
    let open_info = File::open(&path);
    if open_info.is_err() {
        // 如果打开失败，则打印错误信息并终止程序
        println!(
            "无法打开文件 {}: {:?}",
            &path.display(),
            open_info.err().unwrap()
        );
        let default_json="{\"ControllerHost\":\"http://192.168.3.66:10081\",\"ImageHost\":\"http://192.168.3.66:10082\",\"StreamHost\":\"http://192.168.3.66:10083\",\"SystemHtml\":\"# 系统功能介绍 1. 【本地文件扫描】 - 需扫描设置扫描文件类型比如 mp4 mkv avi 等等2. 【Jav刮图信息】  -  需在基础配置中设置URL即JAV首页地址1.根据code抓取相关信息（主图 YY 名称等）制定新文件名称  ```【YY】【Code】Title```2.下载图片并制成JPG和PNG用于列表海报模式与封面模式显示 命名同上3.文件重命名 根据 ```【YY】【Code】Title```建立3级目录方便同YY文件归类3. 【文件重名】功能```【YY】【Code】Title《标签》{{类型}}```（功能2 默认执行此功能）4. 【移动功能】 根据```【YY】【Code】Title```建立3级目录方便同YY文件归类 （功能2 默认执行此功能）5. 【打开文件夹】6. 【打开文件】（使用系统播放器打开）7. 【在线播放】（使用系统播放器打开）8. 【查看原页面】  根据Code 去打开JAV对应网页9. 【下载图片】下载JAV网页的图片列表 并支持查看10. 【删除文件】（当前目录下同名的文件 png jpg 一并删除）11. 【转码】 支持格式转换（考虑磁盘效率 顺序执行，不做多线程） 不能在线播放的文件可执行转码MP4（转码结束默认删除原文件）12. 【文件设置类型】（骑兵 步兵 斯巴达 国产）13. 【文件设置标签】（多个） 标签在基础设置中添加14. 搜索系统使用多服务机制 后台启动3个线程 功能分离 提升性能1、用于页面显示和与后台交互2、用于图片服务加载3、用于流媒体播放即视频在线播放15. 搜索列表的文件操作按钮 支持设置动态定制 16. 【在线播放】（存在不支持的格式 可选择转码）1、支持画中画 全屏 页面全屏 2、记录声音大小3、播放器下方 关联推荐视频（默认同YY作品） 点击即切换播放文件即相关信息并刷新推荐列表4、显示当前文件所有标签（点击刷新关联推荐）5、【更多】按钮 点击展开所有可选的标签（点击刷新关联推荐）6、【隐藏】 后台继续播放 搜索列表下方有悬浮【正在播放】 点击恢复播放页面7、【关闭】 停止播放 关闭播放页面8、【满屏】 点击自适应屏幕大小# 电脑端## 【M系统】 ```点击进入移动端页面```## 【首页】 ```统计```1. 标签数量统计（文件标签） 可快速定位搜索关键词2. 扫描统计（统计扫描盘速度） 可快速定位搜索关键词3. 结果集统计 按照文件类型统计 大小和数量（4. 小文件夹统计（内容放在结果集中 存在才显示）1.如果扫描的目录中存在文件夹小于20M的也同样在结果集中显示处理 支持删除文件夹）## 【文件】 ```文件列表```1.主要用于搜索文件 搜搜【类型】【关键词】 支持排序【名称 时间 大小】 回车键 快捷搜索2、显示 全屏功能 封面模式 海报模式 文件统计 每个文件信息包含布局 主图   左侧 纵向显示所有的标签  右侧按钮 未标记类型时点击选择类型并标记 已标记 则弹窗添加标签YY --点击复制Code--点击复制Size--点击全名称文件名 3、【扫描】重新扫描更新文件库信息4、【重】 搜索code或全名 重复的文件5、【扫描】旁边的数字 点击 弹出窗口 快捷设置搜搜目录与类型 6、【标签】 弹窗 点击 可快速定位搜索关键词7、【历史】 本地记录 搜索过的查询条件 点击可快速定位查询类型与关键词  8、【执行任务】系统启动后所有的转码任务 根据任务状态分开显示9、文件按钮 - 在线播放  - 系统播放 - YY搜索 ---支持定制显示 - 打开文件夹---支持定制显示 - 修改信息---支持定制显示 - 移动---支持定制显示 - 查看图片集（鼠标悬浮 展示全部按钮）---支持定制显示 - 同步（剐蹭JAV）---支持定制显示 - 下图（剐蹭JAV）---支持定制显示 - 封面模式【骑兵 步兵 斯巴达 国产】快捷按钮## 【图鉴】 ```图鉴（YY）```1. 数量倒排 可快速定位搜索关键词## 【设置】 ```文件列表```1. 基础设置1、后台服务URL 用于手机端访问（一般设置当前页面居中的IP即可）端口为：10081：10082：100832、URL和OM-URl 为JAV首页和JAV欧美首页（JAV欧美停更）基础数据设置 需要添加 可选的基础标签 基础类型库 基础扫描文件夹（用于建立数据库后续扫描设置使用 不用每次填写）2. 扫描设置1、选择每次扫描的类型 图片 文件 视频等2、定制搜索页面按钮（全部放出来影响美观）3、每次扫描的目录（灵活定义每次扫描目录）4、标签设置（基础标签库可定义很多 不是每次都使用  可灵活控制页面标签数量，顺序为添加的顺序）3. 系统信息（MarkDown语法编写 在【系统信息中显示】）## 【系统信息】 ```用网页显示设置用系统信息```------------------------------------# 手机端（使用竖屏【手机】的交互方式）## 首页 功能同PC首页 略有删减## 搜索 功能同PC首页 略有删减 但是增加了 简单模式 即只有图片点击进入播放页面但是显示数量更多## 图鉴 功能同PC首页 略有删减## 设置 功能同PC首页 略有删减\",\"Remark\":\"---------------------------------------------------------------------------https://www.javsee.icu/          https://www.seejav.work/  https://www.javsee.in/https://www.javsee.icu/\",\"BaseUrl\":\"https://www.seejav.work/\",\"OMUrl\":\"https://www.javbus.red/123\",\"SelfPath\":\"setting.json\",\"IsJavBus\":false,\"IsDb\":false,\"Tags\":[\"职业\",\"治愈\",\"伦理\",\"秘書\",\"熟女\",\"教師\",\"女上司\",\"超喜歡\",\"医护\",\"誘惑\",\"丝袜\",\"旗袍\",\"绝美\",\"大长腿\",\"泡泡浴\",\"夜店\",\"按摩院\",\"国民女神\",\"竖屏\",\"共演\",\"高跟\",\"女仆\",\"人妻\",\"裙\",\"骚主播\",\"小混血\",\"红衣\",\"古装\",\"三国\",\"剧场版\"],\"TagsLib\":[\"职业\",\"流出\",\"女仆\",\"人妻\",\"治愈\",\"姐妹\",\"伦理\",\"白人\",\"4k\",\"秘書\",\"熟女\",\"教師\",\"女上司\",\"電車\",\"學生\",\"超喜歡\",\"未亡人\",\"異世界\",\"医护\",\"誘惑\",\"丝袜\",\"剧场版\",\"旗袍\",\"Cosplay\",\"经典\",\"東京熱\",\"远古时代\",\"捜査官\",\"巨乳\",\"御姐\",\"萝莉\",\"绝美\",\"空姐\",\"赛车女郎\",\"大长腿\",\"泡泡浴\",\"夜店\",\"按摩院\",\"情趣\",\"调教\",\"国民女神\",\"骚主播\",\"竖屏\",\"三国\",\"新人\",\"共演\",\"小日本\",\"高跟\",\"裙\",\"小混血\",\"红衣\",\"古装\"],\"Dirs\":[\"F:\\\\emby\",\"L:\\\\emby\",\"K:\\\\emby\",\"H:\\\\emby\",\"E:\\\\emby\",\"d:\\\\emby\",\"J:\\\\emby\",\"I:\\\\emby\",\"G:\\\\emby\"],\"DirsLib\":[\"E:\\\\emby\",\"d:\\\\emby\",\"J:\\\\emby\",\"I:\\\\emby\",\"G:\\\\emby\",\"H:\\\\emby\",\"K:\\\\emby\",\"L:\\\\emby\",\"F:\\\\emby\"],\"ImageTypes\":[],\"DocsTypes\":[],\"VideoTypes\":[\"m2ts\",\"vob\",\"ts\",\"iso\",\"mp4\",\"wmv\",\"mkv\",\"avi\",\"Mp4\",\"mpg\"],\"Types\":[\"png\",\"jpg\",\"gif\",\"xlsx\",\"txt\",\"mp4\",\"wmv\",\"mkv\",\"avi\",\"java\",\"xml\",\"m2ts\",\"Mp4\",\"mpg\",\"vob\",\"torrent\",\"iso\",\"ts\"],\"Buttons\":[\"图鉴\",\"更多\",\"转换\",\"详情\",\"文件夹\",\"播放\",\"移动\",\"编辑\",\"剪切\"],\"MovieTypes\":[\"骑兵\",\"步兵\",\"国产\",\"漫动\",\"斯巴达\"]}";
        println!("default_json: {}", default_json);
        // let setting: Setting = match serde_json::from_str(default_json) {
        //     Ok(v) => v,
        //     Err(err) => {
        //         println!(
        //             "serde_json::from_str 错误信息: {:?}  json: {}",
        //             err, default_json
        //         );
        //         Setting::new()
        //     }
        // };
        // refresh_setting(&setting);
        // setting_json = String::from(default_json);
        let mut file = match File::create(path) {
            Err(why) => panic!("couldn't create {:?}: {:?}", &STATIC_SETTING_PATH, why),
            Ok(file) => file,
        };
        // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
        match file.write_all(default_json.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {:?}: {:?}", &STATIC_SETTING_PATH, why)
            }
            Ok(_) => println!("successfully wrote to {}", default_json),
        }
        return loading_file();
    } else {
        open_info
            .ok()
            .unwrap()
            .read_to_string(&mut setting_json)
            .expect("读取文件失败");
    }
    // 将字符串转换为 Setting 对象，如果转换失败则打印错误信息并使用默认的空对象
    let setting: Setting = match serde_json::from_str(&setting_json) {
        Ok(v) => v,
        Err(err) => {
            println!("serde_json::from_str 错误信息: {:?}", err);
            Setting::new()
        }
    };
    // 将请求的对象复制到静态设置的对象中
    STATIC_SETTING.try_lock().unwrap().from(&setting);
    return setting;
}
