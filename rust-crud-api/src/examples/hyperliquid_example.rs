use hyperliquid_rust_sdk::{InfoClient, BaseUrl};
use anyhow::Result;

/// 示例：如何使用 Hyperliquid Rust SDK
/// 
/// 这个示例展示了如何：
/// 1. 创建 InfoClient 实例
/// 2. 获取市场信息
/// 3. 处理 SDK 的基本功能
pub struct HyperliquidService {
    info_client: InfoClient,
}

impl HyperliquidService {
    /// 创建新的 HyperliquidService 实例
    pub async fn new() -> Result<Self> {
        let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;
        Ok(Self { info_client })
    }

    /// 获取所有资产的元数据
    pub async fn get_meta(&self) -> Result<()> {
        match self.info_client.meta().await {
            Ok(meta) => {
                println!("成功获取到 {} 个资产的元数据", meta.universe.len());
                
                // 打印前几个资产的信息
                for (i, asset) in meta.universe.iter().take(5).enumerate() {
                    println!("资产 {}: {} (最大杠杆: {}x)", 
                        i + 1, 
                        asset.name, 
                        asset.max_leverage
                    );
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("获取元数据失败: {}", e);
                Err(e.into())
            }
        }
    }

    /// 获取所有中间价格
    pub async fn get_all_mids(&self) -> Result<()> {
        match self.info_client.all_mids().await {
            Ok(mids) => {
                println!("成功获取到 {} 个资产的中间价格", mids.len());
                
                // 打印前几个价格
                for (asset, price) in mids.iter().take(5) {
                    println!("资产: {}, 中间价格: {}", asset, price);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("获取价格失败: {}", e);
                Err(e.into())
            }
        }
    }
}

/// 运行 Hyperliquid SDK 示例
pub async fn run_hyperliquid_example() -> Result<()> {
    println!("🚀 开始 Hyperliquid SDK 示例");
    
    let service = HyperliquidService::new().await?;
    
    // 获取元数据
    println!("\n📊 获取资产元数据...");
    service.get_meta().await?;
    
    // 获取价格信息
    println!("\n💰 获取价格信息...");
    service.get_all_mids().await?;
    
    println!("\n✅ Hyperliquid SDK 示例完成");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hyperliquid_service() {
        let service = HyperliquidService::new().await;
        
        // 这里可以添加更多的测试
        // 注意：实际测试时可能需要网络连接
        assert!(service.is_ok()); // 验证服务创建成功
    }
}