# Final Report
**Team member**: Huilin Xu (, ), Shanzhi Zhang (, ), Xian Wu (1009735146, amandaxian.wu@mail.utoronto.ca)

# Motivation

As international students and new immigrants, we recognize the challenges newcomers had in finding employment and the urgent need to address this gap. Currently, many new immigrants, students, and marginalized groups are facing some significant disadvantages in finding jobs, not only in Canada, but all around the world. We recently surveyed 200 people and discovered six major obstacles that are preventing them from reaching their employment aspirations:

1. **Language Barriers**: Many non-native speakers find it challenging to create resumes and cover letters that capture the subtleties of the local language and meet professional expectations.
2. **Professional Quality Standards**: Having high-quality, polished, and professional job application materials is essential for finding a job successfully, but it can sometimes feel out of reach for those who don’t have guidance or access to professional services.
3. **Limited Network Access**: Many immigrants find themselves without connections to local professionals in their field, which makes it challenging to access culturally relevant and industry-specific mentorship, results in professional services becoming unaffordable.
4. **Frequent Revisions Needed**: Job application materials, like resumes and cover letters, typically require several rounds of editing. However, mentors and advisors often can't provide the necessary extensive, ongoing support.
5. **Personalization challenges**: Creating application materials requires personal insight, making it hard for individuals to rely only on outside assistance to produce effective documents.
6. **Contextual Barriers**: Being unfamiliar with local industry standards, such as those in Canada, can impact one’s preparedness and potentially lower the success rates in job applications.

These barriers to job searching, caused by limited resources and a lack of knowledge about the position or company, further affect the two most important documents in the application process—the resume and the cover letter. Our project chose to focus on cover letters, partly due to the short length of the program, but more importantly because applicants often overlook the significance of cover letters or struggle to meet expectations. While cover letters are optional in the job application process for some companies, there are still a significant number of positions that require applicants to submit them as an opportunity to demonstrate their understanding of the position and the organization, career experience, and ambition to help the applicant stand out from the pool of candidates. Writing an effective cover letter can be challenging and requires:

1. Overview of job role and responsibilities
2. Alignment with the company's culture and mission
3. Relevant professional experience matching the job description
4. Strong language proficiency
5. Genuine enthusiasm for the role
6. Demonstrated future potential

Therefore combining our own needs as well as those of our target group, the goal of our project is to develop an AI-powered platform designed to increase the competitiveness of users in their job search by producing customized cover letters that meet linguistic and professional standards. Our tool will provide native-level language assistance and industry-appropriate expressions that are customized to each user's unique experience and needs. And with this solution, we hope to reduce the barriers faced by newcomers and marginalized groups, increase their employability, promote economic integration, and contribute to a more inclusive workforce. This AI-driven approach not only meets an unfilled need, but also puts us at the forefront of the market as we scale to support an increasingly globalized and diverse talent pool.

# Objective

The aim of this project is to develop a website integrated with Large Language Models (LLMs) in Rust to assist users in creating professional and customized cover letters, thereby enhancing their job search prospects. The project involves optimizing prompts to improve tool performance and leveraging frameworks to build an interactive, user-friendly website interface.

The project consists of several milestones, each of which is described in detail below.

---

## Front-End Development

### UI/UX Design

Creating an effective full-stack application requires a thoughtful design that takes user habits into account. To lead the development process, we need to prepare a comprehensive UI/UX design document. This document should ensure the interface is intuitive, accessible, and user-friendly. It will outline how users interact with the website and incorporate all possible functionalities, offering a clear roadmap for implementation.

### Website Pages

Based on the UI/UX design, we will implement the website pages in three key stages:

1. **User Initialization**:
    
    Our system will handle personal and private user data. To ensure data security, users must be able to create personal accounts, which will store their chat history and prevent data leakage. This phase includes developing pages for user registration, login, and a personal dashboard.
    
2. **Communication Pages**:
    
    Users will need a space to input essential information, such as job descriptions, desired positions, work experience, and educational background. These pages will facilitate the collection of all relevant data needed to generate tailored cover letters.
    
3. **History Display**:
    
    Recognizing that users may apply for multiple positions and use our tool repeatedly, we plan to provide functionality for reviewing past cover letters. This includes designing pages for accessing and displaying users' history.
    

---

## Back-End Development

The back-end will support all front-end functionalities and consists of the following core sections:

### Core Post and Get Operations

The back-end must enable real-time communication between the front-end and the server. This involves processing user inputs and delivering outputs in real time. It will ensure interactive, real-time functionality for the chatting component of the website.

### Prompt Engineering and Optimization

Generating high-quality, customized cover letters relies heavily on prompt design. Since we intend to use a mature LLM API, optimizing prompts is essential to achieving high performance. This aspect of the back-end involves designing and refining prompts to maximize output quality.

### LLM API Integration

The back-end must integrate the LLM API with optimized prompts to generate personalized cover letters based on the user’s input. Successfully handling input data and composing accurate, professional output to the front-end is a critical milestone for the project.

### Data Storage

To enable users to review their past communications, the back-end will include functionality for securely storing chat histories associated with each user account.

### Back-End Milestones Conclusion:

1. Develop and optimize prompts to achieve high-performance results.
2. Generate high-quality output using the LLM API and optimized prompts.
3. Enable real-time communication between users and the model to tailor outputs to user expectations.
4. Implement secure storage for user chat histories to allow for later review.

---

## Project Objective Milestones Conclusion

The overall development of the project can be divided into the following key milestones:

1. **UI/UX Design**: Create and iteratively improve the design based on user feedback.
2. **Rust Front-End Framework**: Develop the front-end using a Rust framework.
3. **Front-End and Back-End API Interface**: Build a real-time interface for communication between the front-end and back-end.
4. **LLM API Integration**: Successfully incorporate the LLM API with optimized prompts.
5. **Prompt Engineering and Optimization**: Design and refine prompts for high-quality outputs.
6. **Performance Optimization and Testing**: Test the system extensively and make necessary optimizations to ensure optimal performance.

---

