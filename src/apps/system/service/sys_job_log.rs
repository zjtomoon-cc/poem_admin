use crate::apps::common::models::{ListData, PageParams};
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use super::super::entities::{prelude::SysJobLog, sys_job_log};
use super::super::models::sys_job_log::{AddReq, DeleteReq, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SearchReq,
) -> Result<ListData<sys_job_log::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysJobLog::find();
    if let Some(x) = req.job_id {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::JobId.eq(x));
        }
    }
    if let Some(x) = req.job_name {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::JobName.contains(&x));
        }
    }

    if let Some(x) = req.job_group {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::JobGroup.eq(x));
        }
    }
    if let Some(x) = req.is_once {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::IsOnce.eq(x));
        }
    }
    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        s = s.filter(sys_job_log::Column::CreatedAt.gte(x));
    }
    if let Some(x) = req.end_time {
        s = s.filter(sys_job_log::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 分页获取数据
    let paginator = s
        .order_by_desc(sys_job_log::Column::LotId)
        .order_by_desc(sys_job_log::Column::LotOrder)
        .order_by_desc(sys_job_log::Column::CreatedAt)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await.map_err(BadRequest)?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(BadRequest)?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

/// add 添加
pub async fn add<'a, C>(db: &'a C, req: AddReq) -> Result<String>
where
    C: ConnectionTrait<'a>,
{
    let uid = scru128::scru128_string();
    let add_data = sys_job_log::ActiveModel {
        job_log_id: Set(uid.clone()),
        job_id: Set(req.job_id),
        lot_id: Set(req.lot_id),
        lot_order: Set(req.lot_order),
        job_name: Set(req.job_name),
        job_params: Set(req.job_params),
        job_group: Set(req.job_group),
        invoke_target: Set(req.invoke_target),
        status: Set(req.status),
        created_at: Set(req.created_at),
        job_message: Set(req.job_message),
        exception_info: Set(req.exception_info),
        elapsed_time: Set(req.elapsed_time),
        is_once: Set(req.is_once),
    };
    SysJobLog::insert(add_data)
        .exec(db)
        .await
        .map_err(BadRequest)?;

    let res = format!("{}添加成功", uid);

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysJobLog::delete_many();

    s = s.filter(sys_job_log::Column::JobLogId.is_in(delete_req.job_log_ids));

    //开始删除
    let d = s
        .exec(db)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(Error::from_string(
            "你要删除的日志不存在".to_string(),
            StatusCode::BAD_REQUEST,
        )),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// delete 完全删除
pub async fn clean(db: &DatabaseConnection, job_id: String) -> Result<String> {
    let mut s = SysJobLog::delete_many();
    s = s.filter(sys_job_log::Column::JobId.eq(job_id));
    //开始删除
    let d = s
        .exec(db)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;
    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(Error::from_string(
            "你要删除的日志不存在".to_string(),
            StatusCode::BAD_REQUEST,
        )),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, job_log_id: String) -> Result<sys_job_log::Model> {
    let s = SysJobLog::find()
        .filter(sys_job_log::Column::JobLogId.eq(job_log_id))
        .one(db)
        .await
        .map_err(BadRequest)?;

    let res = match s {
        Some(m) => m,
        None => return Err(Error::from_string("没有找到数据", StatusCode::BAD_REQUEST)),
    };
    Ok(res)
}
