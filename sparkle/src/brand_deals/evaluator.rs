use serde::{Deserialize, Serialize};
use lettre::{Message, SmtpTransport, Transport};
use scraper::{Html, Selector};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct BrandDealEvaluator {
    pub company_research: CompanyResearch,
    pub deal_analysis: DealAnalysis,
    pub brand_alignment: BrandAlignment,
    pub email_client: EmailClient,
    pub pending_deals: HashMap<String, BrandDeal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrandDeal {
    pub company_name: String,
    pub offer_details: OfferDetails,
    pub research_results: ResearchResults,
    pub evaluation_status: EvaluationStatus,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchResults {
    pub company_overview: String,
    pub reputation_score: f32,
    pub controversy_check: Vec<Controversy>,
    pub community_fit: f32,
    pub value_alignment: bool,
}

impl BrandDealEvaluator {
    const REVIEW_EMAIL: &'static str = "devteamayo@gmail.com";

    pub async fn evaluate_deal(&mut self, deal: BrandDeal) -> Result<(), String> {
        // Research company
        let research = self.research_company(&deal.company_name).await?;
        
        // Generate report
        let report = self.generate_report(&deal, &research);
        
        // Send for review
        self.submit_for_review(&report).await?;
        
        // Store pending deal
        self.pending_deals.insert(deal.company_name.clone(), deal);
        
        Ok(())
    }

    async fn research_company(&self, company_name: &str) -> Result<ResearchResults, String> {
        let mut results = ResearchResults {
            company_overview: String::new(),
            reputation_score: 0.0,
            controversy_check: Vec::new(),
            community_fit: 0.0,
            value_alignment: false,
        };

        // Basic company research
        results.company_overview = self.company_research.get_overview(company_name).await?;
        
        // Check reputation
        results.reputation_score = self.company_research.check_reputation(company_name).await?;
        
        // Look for controversies
        results.controversy_check = self.company_research.find_controversies(company_name).await?;
        
        // Assess community fit
        results.community_fit = self.assess_community_fit(company_name).await?;
        
        // Check value alignment
        results.value_alignment = self.check_value_alignment(company_name).await?;

        Ok(results)
    }

    fn generate_report(&self, deal: &BrandDeal, research: &ResearchResults) -> String {
        format!(
            "Brand Deal Evaluation Report\n\
            ===========================\n\n\
            Company: {}\n\
            \n\
            Quick Summary:\n\
            - Reputation Score: {}/10\n\
            - Community Fit: {}/10\n\
            - Value Alignment: {}\n\
            \n\
            Company Overview:\n\
            {}\n\
            \n\
            Deal Details:\n\
            - Offer Value: ${}\n\
            - Duration: {} days\n\
            - Requirements: {}\n\
            \n\
            Potential Concerns:\n\
            {}\n\
            \n\
            Community Impact Assessment:\n\
            {}\n\
            \n\
            Recommendation:\n\
            {}\n",
            deal.company_name,
            research.reputation_score,
            research.community_fit,
            if research.value_alignment { "Yes" } else { "No" },
            research.company_overview,
            deal.offer_details.value,
            deal.offer_details.duration.num_days(),
            deal.offer_details.requirements,
            self.format_concerns(&research.controversy_check),
            self.assess_community_impact(deal, research),
            self.generate_recommendation(research)
        )
    }

    async fn submit_for_review(&self, report: &str) -> Result<(), String> {
        let email = Message::builder()
            .from("kamen.sparkle.system@example.com".parse().unwrap())
            .to(Self::REVIEW_EMAIL.parse().unwrap())
            .subject("Brand Deal Evaluation Report - Requires Review")
            .body(report.to_string())
            .unwrap();

        self.email_client.send(email).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn check_approval(&mut self, company_name: &str) -> Result<bool, String> {
        // Check email for response
        if let Some(response) = self.email_client.check_for_response(Self::REVIEW_EMAIL).await? {
            if let Some(deal) = self.pending_deals.get_mut(company_name) {
                deal.evaluation_status = match response.approval {
                    true => EvaluationStatus::Approved,
                    false => EvaluationStatus::Rejected,
                };
                return Ok(response.approval);
            }
        }
        Ok(false)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferDetails {
    pub value: f64,
    pub duration: chrono::Duration,
    pub requirements: String,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Controversy {
    pub description: String,
    pub severity: f32,
    pub date: DateTime<Utc>,
    pub resolved: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvaluationStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
} 